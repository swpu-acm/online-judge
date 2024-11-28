use rocket::{post, serde::json::Json, tokio::fs::remove_dir_all, State};
use serde::{Deserialize, Serialize};
use std::path::Path;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        error::Error,
        organization::CreateOrganization,
        response::{Empty, Response},
    },
    utils::{organization, session},
    Result,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OrgData<'r> {
    pub id: &'r str,
    pub token: &'r str,

    pub org: CreateOrganization,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOrgResponse {
    pub id: String,
}

#[post("/create", data = "<organization>")]
pub async fn create(
    db: &State<Surreal<Client>>,
    organization: Json<OrgData<'_>>,
) -> Result<CreateOrgResponse> {
    if !session::verify(db, organization.id, organization.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    let org = organization::create(db, organization.id, organization.into_inner().org)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::ServerError(Json(
            "Failed to create a new organization".into(),
        )))?;

    Ok(Json(Response {
        success: true,
        message: "Organization created successfully".to_string(),
        data: Some(CreateOrgResponse {
            id: org.id.unwrap().id.to_string(),
        }),
    }))
}

#[post("/delete/<id>", data = "<organization>")]
pub async fn delete(
    db: &State<Surreal<Client>>,
    id: &str,
    organization: Json<OrgData<'_>>,
) -> Result<Empty> {
    if !session::verify(db, organization.id, organization.token).await {
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
    routes![create]
}
