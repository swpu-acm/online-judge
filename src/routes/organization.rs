use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{error::Error, organization::CreateOrganization, response::Response},
    utils::{organization, session},
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
) -> Result<Json<Response<CreateOrgResponse>>, Error> {
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

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create]
}
