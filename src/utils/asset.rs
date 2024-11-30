use std::path::PathBuf;

use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::models::{asset::Asset, UserRecordId};

pub async fn create(
    db: &Surreal<Client>,
    owner: UserRecordId,
    name: &str,
    path: PathBuf,
) -> Result<Option<Asset>> {
    Ok(db
        .create("asset")
        .content(Asset {
            id: None,
            name: name.to_string(),
            owner: owner.into(),
            path,
        })
        .await?)
}

pub async fn get_by_id(db: &Surreal<Client>, id: &str) -> Result<Option<Asset>> {
    Ok(db.select(("asset", id)).await?)
}

pub async fn list_by_owner(db: &Surreal<Client>, owner: UserRecordId) -> Result<Vec<Asset>> {
    Ok(db
        .query("SELECT * FROM asset WHERE owner = $owner")
        .bind(("owner", Into::<Thing>::into(owner)))
        .await?
        .take(0)?)
}

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<Option<Asset>> {
    Ok(db.delete(("asset", id)).await?)
}
