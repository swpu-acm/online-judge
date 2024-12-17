use crate::{
    models::{
        error::Error,
        response::{Empty, Response},
        solution::{CreateSolution, ListSolutions, Solution, UserSolution},
        Credentials, OwnedId,
    },
    utils::{session, solution},
    Result,
};
use rocket::{post, serde::json::Json, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[post("/create", data = "<sol>")]
pub async fn create(db: &State<Surreal<Client>>, sol: Json<CreateSolution<'_>>) -> Result<OwnedId> {
    if !session::verify(db, sol.id, sol.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    let solution = solution::create(db, sol.into_inner())
        .await?
        .ok_or(Error::ServerError(Json("Failed to create solution".into())))?;

    Ok(Json(Response {
        success: true,
        message: "Solution created successfully".to_string(),
        data: Some(OwnedId {
            id: solution.id.unwrap().id.to_string(),
        }),
    }))
}

#[post("/get/<id>", data = "<auth>")]
pub async fn get(
    db: &State<Surreal<Client>>,
    id: &str,
    auth: Json<Option<Credentials<'_>>>,
) -> Result<UserSolution> {
    let solution = solution::get::<Solution>(db, id)
        .await?
        .ok_or(Error::NotFound(Json(
            "Solution with specified id not found".into(),
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

    if authed_id.is_none() {
        return Err(Error::Unauthorized(Json(
            "You have no permission to access this solution".into(),
        )));
    }

    Ok(Json(Response {
        success: true,
        message: "Solution found".to_string(),
        data: Some(solution.into()),
    }))
}

#[post("/update/<id>", data = "<sol>")]
pub async fn update(
    db: &State<Surreal<Client>>,
    id: &str,
    sol: Json<CreateSolution<'_>>,
) -> Result<Empty> {
    if !session::verify(db, sol.id, sol.token).await {
        return Err(Error::Unauthorized(Json("Invalid credentials".into())));
    }

    solution::update(db, id, sol.into_inner())
        .await?
        .ok_or(Error::ServerError(Json(
            "Failed to update solution, please try again later.".into(),
        )))?;

    Ok(Json(Response {
        success: true,
        message: "Solution updated successful".to_string(),
        data: None,
    }))
}

#[post("/list", data = "<data>")]
pub async fn list(db: &State<Surreal<Client>>, data: Json<ListSolutions>) -> Result<Vec<Solution>> {
    let result = solution::list(
        db,
        ("problem".to_string(), data.into_inner().problem).into(),
    )
    .await?;

    if result.is_empty() {
        return Err(Error::NotFound(Json("Solution not found".into())));
    }

    Ok(Json(Response {
        success: true,
        message: "Solution found successfully".to_string(),
        data: Some(result),
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

    Ok(Response {
        success: true,
        message: "Solution deleted successfully".to_string(),
        data: None,
    }
    .into())
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, get, update, list, delete]
}
