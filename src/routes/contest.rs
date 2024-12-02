use rocket::{serde::json::Json, State};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::{
    models::{
        contest::{AddProblems, CreateContest},
        error::Error,
        response::{Empty, Response},
        OwnedId,
    },
    utils::{contest, session},
    Result,
};

#[post("/create", data = "<contest>")]
pub async fn create(db: &State<Surreal<Client>>, contest: Json<CreateContest>) -> Result<OwnedId> {
    if !session::verify(db, &contest.auth.id, &contest.auth.token).await {
        return Err(Error::Unauthorized(Json("Invalid session".into())));
    }

    let contest = contest.into_inner();
    let contest = contest::create(db, &contest.auth.id, contest.data)
        .await
        .map_err(|e| Error::ServerError(Json(e.into())))?
        .ok_or(Error::ServerError(Json("Failed to create contest".into())))?;

    Ok(Json(Response {
        success: true,
        message: "Contest created successfully".into(),
        data: Some(OwnedId {
            id: contest.id.unwrap().id.to_string(),
        }),
    }))
}

#[post("/problems/add", data = "<data>")]
pub async fn add_problem(
    db: &State<Surreal<Client>>,
    data: Json<AddProblems<'_>>,
) -> Result<Empty> {
    if !session::verify(db, &data.auth.id, &data.auth.token).await {
        return Err(Error::Unauthorized(Json("Invalid session".into())));
    }

    let problem = data.into_inner();
    contest::add_problems(
        db,
        problem.contest_id,
        &problem
            .problem_ids
            .iter()
            .map(|&p| Thing::from(("problem", p)))
            .collect::<Vec<Thing>>(),
    )
    .await
    .map_err(|e| Error::ServerError(Json(e.into())))?
    .ok_or(Error::NotFound(Json("Contest not found".into())))?;

    Ok(Json(Response {
        success: true,
        message: "Problems added successfully".into(),
        data: None,
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, add_problem]
}
