use std::path::{Path, PathBuf};

use rocket::{
    form::Form,
    fs::{NamedFile, TempFile},
    get, post, put,
    serde::json::Json,
    tokio::{
        self,
        fs::{create_dir_all, remove_dir_all, File},
    },
    State,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        account::Profile,
        error::{Error, ErrorResponse},
        response::{Empty, Response},
        Record, Token,
    },
    utils::{account, session},
    Result,
};

#[derive(Serialize, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RegisterResponse {
    pub id: String,
    pub token: String,
}

#[post("/create", data = "<register>")]
pub async fn register(
    db: &State<Surreal<Client>>,
    register: Json<RegisterData>,
) -> Result<RegisterResponse> {
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
                data: Some(RegisterResponse { id, token }),
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct MergeProfile<'r> {
    pub id: &'r str,
    pub token: &'r str,
    pub profile: Profile,
}

#[post("/profile", data = "<profile>")]
pub async fn profile(db: &State<Surreal<Client>>, profile: Json<MergeProfile<'_>>) -> Result<Empty> {
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

#[get("/content/<file..>")]
pub async fn content(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("content/").join(file)).await.ok()
}

#[derive(FromForm)]
pub struct Upload<'r> {
    pub id: &'r str,
    pub token: &'r str,
    pub file: TempFile<'r>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadResponse {
    pub uri: String,
    pub path: String,
}

#[put("/content/upload", data = "<data>")]
pub async fn upload_content(
    db: &State<Surreal<Client>>,
    data: Form<Upload<'_>>,
) -> Result<UploadResponse> {
    if !session::verify(db, data.id, data.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant access permission".into(),
        )));
    }

    let file_extension = data
        .file
        .content_type()
        .and_then(|ext| ext.extension().map(ToString::to_string))
        .ok_or_else(|| Error::BadRequest(Json("Invalid file type".into())))?;

    let user_path = std::env::current_dir()
        .unwrap()
        .join("content")
        .join(data.id);

    if !user_path.exists() {
        create_dir_all(&user_path)
            .await
            .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;
    }
    let file_name = format!("{}.{}", uuid::Uuid::new_v4(), file_extension);
    let file_path = user_path.join(&file_name);

    let mut file = data
        .file
        .open()
        .await
        .map_err(|e| Error::ServerError(Json(format!("Failed to open file: {}", e).into())))?;
    let mut output_file = File::create(&file_path)
        .await
        .map_err(|e| Error::ServerError(Json(format!("Failed to create file: {}", e).into())))?;

    tokio::io::copy(&mut file, &mut output_file)
        .await
        .map_err(|e| Error::ServerError(Json(format!("Failed to save file: {}", e).into())))?;

    Ok(Json(Response {
        success: true,
        message: "Content updated successfully".into(),
        data: Some(UploadResponse {
            uri: format!("/account/content/{}/{}", data.id, file_name),
            path: format!("content/{}/{}", data.id, file_name),
        }),
    }))
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

#[derive(Deserialize)]
pub struct Login<'r> {
    pub identity: &'r str,
    pub password: &'r str,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub id: String,
    pub token: String,
}

#[post("/login", data = "<login>")]
pub async fn login(db: &State<Surreal<Client>>, login: Json<Login<'_>>) -> Result<LoginResponse> {
    let session = session::authenticate(db, login.identity, login.password)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::Unauthorized(Json("Invalid credentials".into())))?;
    Ok(Response {
        success: true,
        message: "Login successful".into(),
        data: Some(LoginResponse {
            id: session.account_id.id.to_string(),
            token: session.token.clone(),
        }),
    }
    .into())
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![
        register,
        profile,
        get_profile,
        content,
        upload_content,
        delete,
        login
    ]
}
