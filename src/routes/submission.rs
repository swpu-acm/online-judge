use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{error::Error, response::Response, submission::UserSubmission},
    utils::{session, submission},
    Result,
};

#[derive(Serialize, Deserialize)]
pub struct CreateSubmission<'r> {
    pub id: &'r str,
    pub token: &'r str,

    pub sub: UserSubmission<'r>,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitResponse {
    pub id: String,
}

#[post("/submit", data = "<data>")]
pub async fn submit(
    db: &State<Surreal<Client>>,
    data: Json<CreateSubmission<'_>>,
) -> Result<SubmitResponse> {
    if !session::verify(db, data.id, data.token).await {
        return Err(Error::Unauthorized(Json("Invalid token".into())));
    }

    let submission = submission::create(db, data.id, data.into_inner().sub)
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
