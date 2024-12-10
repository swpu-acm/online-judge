use anyhow::Result;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::solution::{Solution, SolutionData};

pub async fn create(
    db: &Surreal<Client>,
    id: &str,
    data: SolutionData,
) -> Result<Option<Solution>> {
    Ok(db
        .create("solution")
        .content(Solution {
            id: None,

            title: data.title,
            creator: ("account", id).into(),
            problem_id: ("problem".to_string(), data.problem_id).into(),
            content: data.content,

            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<Option<Solution>> {
    Ok(db.delete(("solution", id)).await?)
}

pub async fn get_by_id<M>(db: &Surreal<Client>, id: &str) -> Result<Option<M>>
where
    for<'de> M: Deserialize<'de>,
{
    Ok(db.select(("solution", id)).await?)
}
