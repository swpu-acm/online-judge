use crate::{
    models::{
        category::CreateCategory,
        error::Error,
        response::{Empty, Response},
    },
    utils::{category, session},
    Result,
};
use rocket::{post, serde::json::Json, tokio::fs::remove_dir_all, State};
use serde::{Deserialize, Serialize};
use std::path::Path;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CategoryData<'r> {
    pub id: &'r str,
    pub token: &'r str,

    pub cat: CreateCategory,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCatResponse {
    pub id: String,
}

#[post("/category", data = "<category>")]
pub async fn create(
    db: &State<Surreal<Client>>,
    category: Json<CategoryData<'_>>,
) -> Result<CreateCatResponse> {
    if !session::verify(db, category.id, category.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    let cat = category::create(db, category.id, category.into_inner().cat)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::ServerError(Json("Failed to create category".into())))?;

    Ok(Json(Response {
        success: true,
        message: "Category created successfully".into(),
        data: Some(CreateCatResponse {
            id: cat.id.unwrap().id.to_string(),
        }),
    }))
}

#[post("/category/<id>", data = "<category>")]
pub async fn delete(
    db: &State<Surreal<Client>>,
    id: &str,
    category: Json<CategoryData<'_>>,
) -> Result<Empty> {
    if !session::verify(db, category.id, category.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    category::delete(db, id)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;

    remove_dir_all(Path::new("content/").join(id))
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;

    Ok(Response {
        success: true,
        message: "Category deleted successfully".into(),
        data: None,
    }
    .into())
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, delete]
}
