use anyhow::{Ok, Result};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::models::category::{Category, CategoryData};

pub async fn create(db: &Surreal<Client>, data: CategoryData<'_>) -> Result<Option<Category>> {
    Ok(db
        .create("category")
        .content(Category {
            id: None,
            name: data.name.to_string(),
            owner: data.owner.into(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<Option<Category>> {
    Ok(db.delete(("category", id)).await?)
}

pub async fn list(db: &Surreal<Client>, owner: Thing) -> Result<Vec<Category>> {
    Ok(db
        .query("SELECT * FROM category WHERE owner = $owner")
        .bind(("owner", owner))
        .await?
        .take(0)?)
}
