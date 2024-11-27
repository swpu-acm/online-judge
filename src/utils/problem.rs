use anyhow::Result;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{models::problem::Problem, routes::problem::ProblemData};

pub async fn create(db: &Surreal<Client>, problem: ProblemData<'_>) -> Result<Option<Problem>> {
    Ok(db
        .create("problem")
        .content(Into::<Problem>::into(problem))
        .await?)
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
    Ok(db.delete(("problem", id)).await?)
}

pub async fn get<M>(db: &Surreal<Client>, id: &str) -> Result<Option<M>>
where
    for<'de> M: Deserialize<'de>,
{
    Ok(db.select(("problem", id)).await?)
}

pub async fn list<M>(db: &Surreal<Client>, id: &str) -> Result<Vec<M>>
where
    for<'de> M: Deserialize<'de>,
{
    Ok(db
        .query("SELECT * FROM problem WHERE owner = type::thing(\"account\", $id)")
        .bind(("id", id.to_string()))
        .await?
        .take(0)?)
}
