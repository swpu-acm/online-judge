use eval_stack::compile::Language;
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        error::Error, response::Response, submission::Submission, Credentials, OwnedCredentials,
    },
    utils::{session, submission},
    Result,
};

#[derive(Serialize, Deserialize)]
pub struct CreateSubmission {
    pub auth: OwnedCredentials,
    pub code: String,
    pub lang: Language,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitResponse {
    pub id: String,
}

#[post("/submit/<id>", data = "<data>")]
pub async fn submit(
    db: &State<Surreal<Client>>,
    id: &str,
    data: Json<CreateSubmission>,
) -> Result<SubmitResponse> {
    if !session::verify(db, &data.auth.id, &data.auth.token).await {
        return Err(Error::Unauthorized(Json("Invalid token".into())));
    }

    let data = data.into_inner();
    let submission = submission::create(db, &data.auth.id, id, data.code, data.lang)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::ServerError(Json(
            "Failed to submit, please try again later.".into(),
        )))?;

    Ok(Json(Response {
        success: true,
        message: "Submission created successfully".to_string(),
        data: Some(SubmitResponse {
            id: submission.id.unwrap().id.to_string(),
        }),
    }))
}

#[post("/get/<id>", data = "<_auth>")]
pub async fn get(
    db: &State<Surreal<Client>>,
    id: &str,
    _auth: Json<Credentials<'_>>,
) -> Result<Submission> {
    let submission = submission::get_by_id(db, id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::NotFound(Json("Submission not found".into())))?;

    Ok(Json(Response {
        success: true,
        message: "Submission fetched successfully".to_string(),
        data: Some(submission),
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![submit, get]
}
