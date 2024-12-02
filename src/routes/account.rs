use std::path::Path;

use rocket::{get, post, serde::json::Json, tokio::fs::remove_dir_all, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        account::{Login, MergeProfile, Profile, Register},
        error::{Error, ErrorResponse},
        response::{Empty, Response},
        OwnedCredentials, Record, Token,
    },
    utils::{account, session},
    Result,
};

#[post("/create", data = "<register>")]
pub async fn register(
    db: &State<Surreal<Client>>,
    register: Json<Register>,
) -> Result<OwnedCredentials> {
    match account::create(db, register.into_inner()).await {
        Ok(Some(account)) => {
            let token = match session::create(db, account.id.clone().unwrap()).await {
                Ok(session) => session.unwrap().token,
                Err(e) => return Err(Error::ServerError(Json(e.to_string().into()))),
            };
            let id = account.id.unwrap().id.to_string();
            Ok(Response {
                success: true,
                message: format!("Account with id {} created successfully", &id),
                data: Some(OwnedCredentials { id, token }),
            }
            .into())
        }
        Ok(None) => Ok(Response {
            success: false,
            message: "Specified username or email already exists".to_string(),
            data: None,
        }
        .into()),
        Err(e) => Err(Error::ServerError(
            ErrorResponse {
                success: false,
                message: e.to_string(),
            }
            .into(),
        )),
    }
}

#[post("/profile", data = "<profile>")]
pub async fn profile(
    db: &State<Surreal<Client>>,
    profile: Json<MergeProfile<'_>>,
) -> Result<Empty> {
    account::get_by_id::<Record>(db, profile.id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::NotFound(Json("Account not found".into())))?;

    if session::verify(db, profile.id, profile.token).await {
        account::merge_profile(db, profile.id, profile.profile.clone())
            .await
            .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;
        Ok(Response {
            success: true,
            message: "Profile updated successfully".into(),
            data: None,
        }
        .into())
    } else {
        Err(Error::Unauthorized(Json("Invalid token".into())))
    }
}

#[get("/profile/<id>")]
pub async fn get_profile(db: &State<Surreal<Client>>, id: &str) -> Result<Profile> {
    let profile = account::get_by_identity::<Profile>(db, id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::NotFound(Json("Account not found".into())))?;

    Ok(Response {
        success: true,
        message: "Profile fetched successfully".into(),
        data: Some(profile),
    }
    .into())
}

#[post("/delete/<id>", data = "<auth>")]
pub async fn delete(db: &State<Surreal<Client>>, id: &str, auth: Json<Token<'_>>) -> Result<Empty> {
    if !session::verify(db, id, auth.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant access permission".into(),
        )));
    }

    account::delete(db, id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;

    remove_dir_all(Path::new("content/").join(id))
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;

    Ok(Response {
        success: true,
        message: "Account deleted successfully".into(),
        data: None,
    }
    .into())
}

#[post("/login", data = "<login>")]
pub async fn login(
    db: &State<Surreal<Client>>,
    login: Json<Login<'_>>,
) -> Result<OwnedCredentials> {
    let session = session::authenticate(db, login.identity, login.password)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::Unauthorized(Json("Invalid credentials".into())))?;
    Ok(Response {
        success: true,
        message: "Login successful".into(),
        data: Some(OwnedCredentials {
            id: session.account_id.id.to_string(),
            token: session.token.clone(),
        }),
    }
    .into())
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![register, profile, get_profile, delete, login]
}
