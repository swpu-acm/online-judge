use crate::{
    models::{
        category::{Category, CreateCategory, ListCategories},
        error::Error,
        response::{Empty, Response},
        OwnedId,
    },
    utils::{category, session},
    Result,
};
use rocket::{post, serde::json::Json, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[post("/create", data = "<category>")]
pub async fn create(
    db: &State<Surreal<Client>>,
    category: Json<CreateCategory<'_>>,
) -> Result<OwnedId> {
    if !session::verify(db, category.id, category.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    let data = category::create(db, category.into_inner().data)
        .await?
        .ok_or(Error::ServerError(Json("Failed to create category".into())))?;

    Ok(Json(Response {
        success: true,
        message: "Category created successfully".into(),
        data: Some(OwnedId {
            id: data.id.unwrap().id.to_string(),
        }),
    }))
}

#[post("/delete/<id>", data = "<category>")]
pub async fn delete(
    db: &State<Surreal<Client>>,
    id: &str,
    category: Json<CreateCategory<'_>>,
) -> Result<Empty> {
    if !session::verify(db, category.id, category.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    category::delete(db, id).await?;

    Ok(Response {
        success: true,
        message: "Category deleted successfully".into(),
        data: None,
    }
    .into())
}

#[post("/list", data = "<data>")]
pub async fn list(
    db: &State<Surreal<Client>>,
    data: Json<ListCategories>,
) -> Result<Vec<Category>> {
    let result = category::list(db, data.into_inner().owner.into())
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;

    if result.is_empty() {
        return Err(Error::NotFound(Json("Category not found".into())));
    }

    Ok(Json(Response {
        success: true,
        message: "Category found successfully".to_string(),
        data: Some(result),
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![create, delete, list]
}
