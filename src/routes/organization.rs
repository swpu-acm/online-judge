use rocket::{post, serde::json::Json, tokio::fs::remove_dir_all, State};
use std::path::Path;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        error::Error,
        organization::CreateOrganization,
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

    let org = organization::create(db, org.id, org.into_inner().org)
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

    organization::delete(db, id).await?;

    remove_dir_all(Path::new("content/").join(id)).await?;
    
    Ok(Response {
        success: true,
        message: "Organization deleted successfully".into(),
        data: None,
    }
    .into())
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, delete]
}
