use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::problem::Problem;

pub async fn create(db: &Surreal<Client>, problem: Problem) -> Result<Option<Problem>> {
    Ok(db.create("problem").content(problem).await?)
}

pub async fn update(db: &Surreal<Client>, problem: Problem) -> Result<Option<Problem>> {
    Ok(db
        .update((
            "problem",
            problem.id.clone().expect("empty id").id.to_string(),
        ))
        .content(problem)
        .await?)
}

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<Option<Problem>> {
    Ok(db.delete(("problem", id.to_string())).await?)
}

pub async fn get(db: &Surreal<Client>, id: &str) -> Result<Option<Problem>> {
    Ok(db.select(("problem", id.to_string())).await?)
}
