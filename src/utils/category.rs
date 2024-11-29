use anyhow::{Ok, Result};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::models::category::{Category, CreateCategory};

pub async fn create(
    db: &Surreal<Client>,
    id: &str,
    cat: CreateCategory,
) -> Result<Option<Category>> {
    Ok(db
        .create("category")
        .content(Category {
            id: None,
            name: cat.name,
            owner: (cat.group, id.to_string()).into(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<Option<Category>> {
    Ok(db.delete(("category", id)).await?)
}

pub async fn get_by_owner(db: &Surreal<Client>, owner: Thing) -> Result<Vec<Category>> {
    Ok(db
        .query("SELECT * FROM category WHERE owner = $owner")
        .bind(("owner", owner))
        .await?
        .take(0)?)
}

pub async fn get_by_name(db: &Surreal<Client>, name: String) -> Result<Option<Category>> {
    Ok(db
        .query("SELECT * FROM category WHERE name = $name")
        .bind(("name", name))
        .await?
        .take(0)?)
}
