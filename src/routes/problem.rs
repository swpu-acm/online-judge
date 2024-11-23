use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        error::Error,
        problem::{Mode, Problem, Sample},
        response::Response,
    },
    utils::{problem, session},
    Result,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProblemData<'r> {
    pub id: &'r str,
    pub token: &'r str,

    pub title: &'r str,
    pub description: &'r str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    pub samples: Vec<Sample>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,

    pub time_limit: i32,
    pub memory_limit: i32,
    pub test_cases: Vec<Sample>,

    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub mode: Mode,
    pub private: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProblemResponse {
    pub id: String,
}

#[post("/create", data = "<problem>")]
pub async fn create(
    db: &State<Surreal<Client>>,
    problem: Json<ProblemData<'_>>,
) -> Result<ProblemResponse> {
    if !session::verify(db, problem.id, problem.token).await {
        return Err(Error::Unauthorized(Json("Invalid token".into())));
    }

    let problem = problem::create(db, problem.into_inner())
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::ServerError(Json(
            "Failed to create problem, please try again later.".into(),
        )))?;

    Ok(Json(Response {
        success: true,
        message: "Problem created successfully".to_string(),
        data: Some(ProblemResponse {
            id: problem.id.unwrap().id.to_string(),
        }),
    }))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Authenticate<'r> {
    pub id: &'r str,
    pub token: &'r str,
}

#[post("/get", data = "<auth>")]
pub async fn get(db: &State<Surreal<Client>>, auth: Json<Authenticate<'_>>) -> Result<Problem> {
    let problem = problem::get(db, auth.id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::NotFound(Json(
            "Problem with specified id not found".into(),
        )))?;

    if problem.private && !session::verify(db, auth.id, auth.token).await {
        return Err(Error::Unauthorized(Json(
            "You have no permission to access this problem".into(),
        )));
    }

    Ok(Json(Response {
        success: true,
        message: "Problem found".to_string(),
        data: Some(problem),
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, get]
}
