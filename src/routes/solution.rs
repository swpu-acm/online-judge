use crate::{
    models::{
        error::Error,
        response::{Empty, Response},
        solution::CreateSolution,
        Credentials, OwnedId,
    },
    utils::{session, solution},
    Result,
};
use rocket::{post, serde::json::Json, tokio::fs::remove_dir_all, State};
use std::path::Path;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[post("/create", data = "<sol>")]
pub async fn create(db: &State<Surreal<Client>>, sol: Json<CreateSolution<'_>>) -> Result<OwnedId> {
    if !session::verify(db, sol.id, sol.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    let solution = solution::create(db, sol.id, sol.into_inner().data)
        .await?
        .ok_or(Error::ServerError(Json("Failed to create solution".into())))?;

    Ok(Json(Response {
        success: true,
        message: "Solution created successfully".to_string(),
        data: Some(OwnedId {
            id: solution.id.unwrap().to_string(),
        }),
    }))
}

#[post("/delete/<id>", data = "<sol>")]
pub async fn delete(
    db: &State<Surreal<Client>>,
    id: &str,
    sol: Json<Credentials<'_>>,
) -> Result<Empty> {
    if !session::verify(db, sol.id, sol.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    solution::delete(db, id).await?;

    remove_dir_all(Path::new("content/").join(id)).await?;

    Ok(Response {
        success: true,
        message: "Solution deleted successfully".to_string(),
        data: None,
    }
    .into())
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, delete]
}
