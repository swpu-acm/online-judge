use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::solution::{Solution, SolutionData};

pub async fn create(
    db: &Surreal<Client>,
    id: &str,
    data: SolutionData<'_>,
) -> Result<Option<Solution>> {
    Ok(db
        .create("solution")
        .content(Solution {
            id: None,

            title: data.title.to_string(),
            creator: ("account", id).into(),
            problem: ("problem", data.problem).into(),
            content: data.content.to_string(),

            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<Option<Solution>> {
    Ok(db.delete(("solution", id)).await?)
}
