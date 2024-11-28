use std::time::Duration;

use eval_stack::{compile::Language, config::JudgeOptions, judge::JudgeStatus};
use rocket::{
    serde::json::Json,
    tokio::{
        fs::{create_dir_all, File},
        io::AsyncWriteExt,
    },
    State,
};
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
pub struct ProblemData<'r> {
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

    // pub owner: Thing,
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
    pub id: Option<&'r str>,
    pub token: Option<&'r str>,
}

#[post("/get/<id>", data = "<auth>")]
pub async fn get(
    db: &State<Surreal<Client>>,
    id: &str,
    auth: Json<Authenticate<'_>>,
) -> Result<ProblemDetail> {
    let problem = problem::get::<ProblemDetail>(db, id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::NotFound(Json(
            "Problem with specified id not found".into(),
        )))?;

    let has_permission = if problem.private {
        if auth.id.is_none() || auth.token.is_none() {
            false
        } else {
            if !session::verify(db, auth.id.unwrap(), auth.token.unwrap()).await {
                false
            } else {
                auth.id.unwrap() == problem.owner.id.to_string()
            }
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
        data: Some(problem),
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

    let problems = problem::list_for_account(db, data.id, authed_id, data.limit)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;

    Ok(Json(Response {
        success: true,
        message: "Problems found".to_string(),
        data: Some(problems),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct SubmitData<'r> {
    pub id: &'r str,
    pub token: &'r str,
    pub language: &'r str,
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitResponse {
    pub status: String,
}

#[post("/submit/<id>", data = "<data>")]
pub async fn submit(
    db: &State<Surreal<Client>>,
    id: &str,
    data: Json<SubmitData<'_>>,
) -> Result<SubmitResponse> {
    if !session::verify(db, data.id, data.token).await {
        return Err(Error::Unauthorized(Json("Invalid token".into())));
    }

    let problem = problem::get::<Problem>(db, id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::NotFound(Json(
            "Problem with specified id not found".into(),
        )))?;

    let options = JudgeOptions {
        time_limit: Duration::from_secs(problem.time_limit),
        memory_limit: problem.memory_limit,
    };

    let workspace = std::env::current_dir()
        .unwrap()
        .join("content")
        .join(data.id)
        .join("workspace")
        .join(uuid::Uuid::new_v4().to_string());

    if !workspace.exists() {
        create_dir_all(&workspace)
            .await
            .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;
    }

    let language = match data.language {
        "rust" => Language::Rust,
        "c" => Language::C,
        "cpp" => Language::CPP,
        "python" => Language::Python,
        _ => Language::Rust,
    };

    let source_file_path = workspace.join("source.code");
    File::create(&source_file_path)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .write_all(data.code.as_bytes())
        .await
        .unwrap();

    println!("{:?}", problem.test_cases);
    let result = eval_stack::case::run_test_cases(
        language,
        workspace,
        source_file_path,
        options,
        problem
            .test_cases
            .iter()
            .map(|s| (s.input.clone(), s.output.clone()))
            .collect(),
        true,
    )
    .await
    .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;
    println!("{:?}", result);

    let err = result
        .iter()
        .find(|&r| !matches!(r.status, JudgeStatus::Accepted));

    if let Some(_e) = err {
        Ok(Json(Response {
            success: true,
            message: "Unaccepted".to_string(),
            data: Some(SubmitResponse {
                status: "Wrong Answer".to_string(),
            }),
        }))
    } else {
        Ok(Json(Response {
            success: true,
            message: "Accepted".to_string(),
            data: Some(SubmitResponse {
                status: "Accepted".to_string(),
            }),
        }))
    }
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, get, submit, list]
}
