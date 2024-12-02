use rocket::{
    form::Form,
    fs::NamedFile,
    serde::json::Json,
    tokio::fs::{create_dir_all, File},
    State,
};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        asset::{AssetFile, CreateAsset, UserContent},
        error::Error,
        response::{Empty, Response},
        Credentials,
    },
    utils::{asset, session},
    Result,
};

#[put("/upload", format = "multipart/form-data", data = "<data>")]
pub async fn upload(
    db: &State<Surreal<Client>>,
    data: Form<CreateAsset<'_>>,
) -> Result<UserContent> {
    if !session::verify(db, data.auth.id, data.auth.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant access permission".into(),
        )));
    }

    let file_extension = data
        .file
        .content_type()
        .and_then(|ext| ext.extension().map(ToString::to_string))
        .ok_or_else(|| Error::BadRequest(Json("Invalid file type".into())))?;

    let user_path = std::env::current_dir()
        .unwrap()
        .join("content")
        .join(data.auth.id);

    if !user_path.exists() {
        create_dir_all(&user_path)
            .await
            .map_err(|e| Error::ServerError(Json(e.to_string().into())))?;
    }
    let file_name = format!("{}.{}", uuid::Uuid::new_v4(), file_extension);
    let file_path = user_path.join(&file_name);

    let mut file = data
        .file
        .open()
        .await
        .map_err(|e| Error::ServerError(Json(format!("Failed to open file: {}", e).into())))?;
    let mut output_file = File::create(&file_path)
        .await
        .map_err(|e| Error::ServerError(Json(format!("Failed to create file: {}", e).into())))?;

    rocket::tokio::io::copy(&mut file, &mut output_file)
        .await
        .map_err(|e| Error::ServerError(Json(format!("Failed to save file: {}", e).into())))?;

    let asset = asset::create(
        db,
        data.owner.clone(),
        data.file
            .name()
            .unwrap_or(&uuid::Uuid::new_v4().to_string()),
        file_path,
    )
    .await
    .map_err(|e| Error::ServerError(Json(e.into())))?
    .ok_or(Error::ServerError(Json("Failed to create asset".into())))?;
    Ok(Json(Response {
        success: true,
        message: "Content updated successfully".into(),
        data: Some(UserContent {
            id: asset.id.unwrap().id.to_string(),
        }),
    }))
}

#[get("/<id>")]
pub async fn get(db: &State<Surreal<Client>>, id: &str) -> Option<AssetFile> {
    let asset = asset::get_by_id(db, id).await.ok()??;

    Some(AssetFile(NamedFile::open(&asset.path).await.ok()?))
}

#[post("/delete/<id>", data = "<auth>")]
pub async fn delete(
    db: &State<Surreal<Client>>,
    id: &str,
    auth: Form<Credentials<'_>>,
) -> Result<Empty> {
    if !session::verify(db, auth.id, auth.token).await {
        return Err(Error::Unauthorized(Json(
            "Failed to grant access permission".into(),
        )));
    }

    let asset = asset::get_by_id(db, id)
        .await
        .map_err(Error::from)?
        .ok_or(Error::NotFound(Json("Asset not found".into())))?;

    rocket::tokio::fs::remove_file(&asset.path)
        .await
        .map_err(|e| Error::ServerError(Json(format!("Failed to delete file: {}", e).into())))?;

    Ok(Json(Response {
        success: true,
        message: "Content deleted successfully".into(),
        data: None,
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    use rocket::routes;
    routes![upload, get, delete]
}
