use crate::{
    models::{
        category::Category,
        error::Error,
        response::{Empty, Response},
    },
    utils::{category, session},
    Result,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CatData {
    pub id: &'r str,
    pub token: &'r str,

    pub cat: Category,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCatResponse {
    pub id: String,
    pub group: String,
}

#[post("/category", data = "<category>")]
pub async fn create(
    db: &State<Surreal<Client>>,
    category: Json<CatData<'_>>,
) -> Result<CreateCatResponse> {
    if !session::verify(db, category.id, category.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant permission".into(),
        )));
    }

    let cat = category::create(db, category.id, category.into_inner().cat)
        .await
        .map_err(|e| Error::ServerError(Json(e.to_string().into())))?
        .ok_or(Error::ServerError(Json("Failed to create category".into(),
    )))?;

    Ok(Json(Response {
        success: true,
        message: "Category created successfully".into(),
        data: Some(CreateCatResponse {
            id: cat.id.unwrap().id.to_string(),    
        }),
    }))
}
