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
    models::{error::Error, problem::Problem, response::Response},
    utils::{problem, session},
    Result,
};

#[derive(Serialize, Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum Status {
    InQueue,
    Judging,
    Ready,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SubmitData<'r> {
    pub id: &'r str,
    pub token: &'r str,
    pub lang: &'r str,
    pub problem_id: &'r str,
    pub code: String,
    pub status: Status,
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

    let language = match data.lang {
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
    routes![submit]
}
