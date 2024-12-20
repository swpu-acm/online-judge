use rocket::{post, serde::json::Json, tokio::fs::remove_dir_all, State};
use std::path::Path;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        error::Error,
        organization::{ChangeMember, CreateOrganization, Organization, UserOrganization},
        response::{Empty, Response},
        Credentials, OwnedId,
    },
    utils::{organization, session},
    Result,
};

#[post("/create", data = "<org>")]
pub async fn create(
    db: &State<Surreal<Client>>,
    org: Json<CreateOrganization<'_>>,
) -> Result<OwnedId> {
    if !session::verify(db, org.id, org.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    let org = organization::create(db, org.into_inner())
        .await?
        .ok_or(Error::ServerError(Json(
            "Failed to create a new organization".into(),
        )))?;

    Ok(Json(Response {
        success: true,
        message: "Organization created successfully".to_string(),
        data: Some(OwnedId {
            id: org.id.unwrap().id.to_string(),
        }),
    }))
}

#[post("/get/<id>", data = "<auth>")]
pub async fn get(
    db: &State<Surreal<Client>>,
    id: &str,
    auth: Json<Option<Credentials<'_>>>,
) -> Result<UserOrganization> {
    if let Some(auth) = auth.into_inner() {
        if !session::verify(db, auth.id, auth.token).await {
            return Err(Error::Unauthorized(Json(
                "Failed to grant permission".into(),
            )));
        }
    } else {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    let org = organization::get_by_id::<Organization>(db, id)
        .await?
        .ok_or(Error::NotFound(Json("Organization not found".into())))?;

    Ok(Json(Response {
        success: true,
        message: "Organization found".to_string(),
        data: Some(org.into()),
    }))
}

#[post("/add/<id>", data = "<member>")]
pub async fn add(
    db: &State<Surreal<Client>>,
    id: &str,
    member: Json<ChangeMember<'_>>,
) -> Result<Empty> {
    if !session::verify(db, member.id, member.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    organization::add(db, id, member.into_inner().members)
        .await?
        .ok_or(Error::ServerError(Json(
            "Failed to add members to the organization".into(),
        )))?;

    Ok(Json(Response {
        success: true,
        message: "Members added successfully".into(),
        data: None,
    }))
}

#[post("/remove/<id>", data = "<member>")]
pub async fn remove(
    db: &State<Surreal<Client>>,
    id: &str,
    member: Json<ChangeMember<'_>>,
) -> Result<Empty> {
    if !session::verify(db, member.id, member.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    organization::remove(db, id, member.into_inner().members)
        .await?
        .ok_or(Error::ServerError(Json(
            "Failed to remove members to the organization".into(),
        )))?;

    Ok(Json(Response {
        success: true,
        message: "Members removed successfully".into(),
        data: None,
    }))
}

#[post("/update/<id>", data = "<org>")]
pub async fn update(
    db: &State<Surreal<Client>>,
    id: &str,
    org: Json<CreateOrganization<'_>>,
) -> Result<Empty> {
    if !session::verify(db, org.id, org.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    organization::update(db, id, org.into_inner().org)
        .await?
        .ok_or(Error::ServerError(Json(
            "Failed to update the organization".into(),
        )))?;

    Ok(Json(Response {
        success: true,
        message: "Organization updated successfully".to_string(),
        data: None,
    }))
}

#[post("/delete/<id>", data = "<org>")]
pub async fn delete(
    db: &State<Surreal<Client>>,
    id: &str,
    org: Json<Credentials<'_>>,
) -> Result<Empty> {
    if !session::verify(db, org.id, org.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    organization::delete(db, id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;

    remove_dir_all(Path::new("content/").join(id))
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;

    Ok(Response {
        success: true,
        message: "Organization deleted successfully".into(),
        data: None,
    }
    .into())
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, add, update, get, delete, remove]
}
