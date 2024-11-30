use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        error::Error,
        problem::{Mode, Problem, ProblemDetail, Sample},
        response::Response,
        OwnedCredentials,
    },
    utils::{problem, session},
    Result,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateProblem<'r> {
    pub id: &'r str,
    pub token: &'r str,

    pub title: &'r str,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    pub samples: Vec<Sample>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,

    pub time_limit: u64,
    pub memory_limit: u64,
    pub test_cases: Vec<Sample>,

    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub mode: Mode,
    pub private: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ProblemResponse {
    pub id: String,
}

#[post("/create", data = "<problem>")]
pub async fn create(
    db: &State<Surreal<Client>>,
    problem: Json<CreateProblem<'_>>,
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
    pub id: Option<&'r str>,
    pub token: Option<&'r str>,
}

#[post("/get/<id>", data = "<auth>")]
pub async fn get(
    db: &State<Surreal<Client>>,
    id: &str,
    auth: Json<Authenticate<'_>>,
) -> Result<ProblemDetail> {
    let problem = problem::get::<Problem>(db, id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::NotFound(Json(
            "Problem with specified id not found".into(),
        )))?;

    let has_permission = if problem.private {
        if auth.id.is_none()
            || auth.token.is_none()
            || !session::verify(db, auth.id.unwrap(), auth.token.unwrap()).await
        {
            false
        } else {
            auth.id.unwrap() == problem.owner.id.to_string()
        }
    } else {
        true
    };

    if !has_permission {
        return Err(Error::Unauthorized(Json(
            "You have no permission to access this problem".into(),
        )));
    }

    Ok(Json(Response {
        success: true,
        message: "Problem found".to_string(),
        data: Some(problem.into()),
    }))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ListProblem {
    pub id: Option<String>,
    pub auth: Option<OwnedCredentials>,
    pub limit: Option<u32>,
}

#[post("/list", data = "<data>")]
pub async fn list(
    db: &State<Surreal<Client>>,
    data: Json<ListProblem>,
) -> Result<Vec<ProblemDetail>> {
    let authed_id = if let Some(auth) = &data.auth {
        if !session::verify(db, &auth.id, &auth.token).await {
            return Err(Error::Unauthorized(Json("Invalid token".into())));
        };
        Some(auth.id.clone())
    } else {
        None
    };

    let data = data.into_inner();

    let problems = problem::list_for_account::<Problem>(db, data.id, authed_id, data.limit)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;

    Ok(Json(Response {
        success: true,
        message: "Problems found".to_string(),
        data: Some(problems.into_iter().map(|p| p.into()).collect()),
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, get, list]
}
