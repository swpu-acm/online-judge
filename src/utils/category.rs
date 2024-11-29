use anyhow::Result;
use crate::models::category::{Category, CreateCategory};

pub async fn create(
    db: &Surreal<Client>,
    id: &str,
    cat: CreateCategory,
) -> Result<Option<Category>> {
    Ok(db
        .create("category")
        .content(category {
            id: None,
            name: cat.name,
            owner: (cat.group, id).into(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}
