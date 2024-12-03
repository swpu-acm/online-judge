use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        account::Account,
        error::Error,
        problem::{CreateProblem, Problem, ProblemVisibility, UserProblem},
        response::Response,
        Credentials, OwnedCredentials, OwnedId,
    },
    utils::{account, problem, session},
    Result,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ProblemResponse {
    pub id: String,
}

#[post("/create", data = "<problem>")]
pub async fn create(
    db: &State<Surreal<Client>>,
    problem: Json<CreateProblem<'_>>,
) -> Result<OwnedId> {
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
        data: Some(OwnedId {
            id: problem.id.unwrap().id.to_string(),
        }),
    }))
}

#[post("/get/<id>", data = "<auth>")]
pub async fn get(
    db: &State<Surreal<Client>>,
    id: &str,
    auth: Json<Option<Credentials<'_>>>,
) -> Result<UserProblem> {
    let problem = problem::get::<Problem>(db, id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::NotFound(Json(
            "Problem with specified id not found".into(),
        )))?;

    let authed_id = if let Some(auth) = auth.into_inner() {
        if !session::verify(db, auth.id, auth.token).await {
            return Err(Error::Unauthorized(Json("Invalid credentials".into())));
        } else {
            Some(auth.id)
        }
    } else {
        None
    };

    let has_permission = match problem.visibility {
        ProblemVisibility::ContestOnly => {
            if !authed_id.is_some() {
                false
            } else {
                // Check for contest access
                todo!()
            }
        }
        ProblemVisibility::Public => true,
        ProblemVisibility::Private => {
            if !authed_id.is_some() {
                false
            } else {
                problem.owner.id.to_string() == authed_id.unwrap()
            }
        }
        ProblemVisibility::Internal => {
            if !authed_id.is_some() {
                false
            } else {
                // Check for internal access
                todo!()
            }
        }
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
    pub identity: Option<String>,
    pub auth: Option<OwnedCredentials>,
    pub limit: Option<u32>,
}

#[post("/list", data = "<data>")]
pub async fn list(
    db: &State<Surreal<Client>>,
    data: Json<ListProblem>,
) -> Result<Vec<UserProblem>> {
    let authed_id = if let Some(auth) = &data.auth {
        if !session::verify(db, &auth.id, &auth.token).await {
            return Err(Error::Unauthorized(Json("Invalid token".into())));
        };
        Some(auth.id.clone())
    } else {
        None
    };

    let data = data.into_inner();

    let account_id = if let Some(identity) = data.identity.clone() {
        Some(
            account::get_by_identity::<Account>(db, &identity)
                .await
                .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
                .ok_or(Error::Unauthorized(Json("Invalid identity".into())))?
                .id
                .unwrap()
                .id
                .to_string(),
        )
    } else {
        None
    };

    let problems = problem::list_for_account::<Problem>(db, account_id, authed_id, data.limit)
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
