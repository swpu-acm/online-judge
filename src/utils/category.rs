use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, Surreal};

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
