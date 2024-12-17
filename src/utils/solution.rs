use anyhow::Result;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::models::solution::{CreateSolution, Solution};

pub async fn create(db: &Surreal<Client>, sol: CreateSolution<'_>) -> Result<Option<Solution>> {
    Ok(db
        .create("solution")
        .content(Into::<Solution>::into(sol))
        .await?)
}

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<Option<Solution>> {
    Ok(db.delete(("solution", id)).await?)
}

pub async fn get<M>(db: &Surreal<Client>, id: &str) -> Result<Option<M>>
where
    for<'de> M: Deserialize<'de>,
{
    Ok(db.select(("solution", id)).await?)
}

pub async fn list(db: &Surreal<Client>, problem: Thing) -> Result<Vec<Solution>> {
    Ok(db
        .query("SELECT * FROM solution WHERE problem = $problem")
        .bind(("problem", problem))
        .await?
        .take(0)?)
}

pub async fn update(
    db: &Surreal<Client>,
    id: &str,
    solution: CreateSolution<'_>,
) -> Result<Option<Solution>> {
    Ok(db
        .update(("solution", id))
        .content(Into::<Solution>::into(solution))
        .await?)
}
