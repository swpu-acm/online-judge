use rocket::{serde::json::Json, State};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::{
    models::{
        contest::{AddProblems, CreateContest, UserContest},
        error::Error,
        response::{Empty, Response},
        Credentials, OwnedId,
    },
    utils::{contest, session},
    Result,
};

#[post("/create", data = "<contest>")]
pub async fn create(db: &State<Surreal<Client>>, contest: Json<CreateContest>) -> Result<OwnedId> {
    if !session::verify(db, &contest.auth.id, &contest.auth.token).await {
        return Err(Error::Unauthorized(Json("Invalid credentials".into())));
    }

    let contest = contest.into_inner();
    let contest = contest::create(db, &contest.auth.id, contest.data)
        .await?
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
pub async fn add_problems(
    db: &State<Surreal<Client>>,
    data: Json<AddProblems<'_>>,
) -> Result<Empty> {
    if !session::verify(db, &data.auth.id, &data.auth.token).await {
        return Err(Error::Unauthorized(Json("Invalid credentials".into())));
    }

    let problem = data.into_inner();

    Ok(Json(Response {
        success: true,
        message: "Problems added successfully".into(),
        data: None,
    }))
}

#[post("/list/all", data = "<auth>")]
pub async fn list_all(
    db: &State<Surreal<Client>>,
    auth: Json<Credentials<'_>>,
) -> Result<Vec<UserContest>> {
    if !session::verify(db, &auth.id, &auth.token).await {
        return Err(Error::Unauthorized(Json("Invalid credentials".into())));
    }

    let contests = contest::list_all(db).await?;
    Ok(Json(Response {
        success: true,
        message: "Contests listed successfully".into(),
        data: Some(contests.into_iter().map(|c| c.into()).collect()),
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, add_problems, list_all]
}
