use anyhow::Result;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::problem::{CreateProblem, Problem};

pub async fn create(db: &Surreal<Client>, problem: CreateProblem<'_>) -> Result<Option<Problem>> {
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

const LIST_QUERY: &str = r#"
IF $authed THEN
    IF $limit THEN
        SELECT * FROM problem WHERE owner = type::thing("account", $id) LIMIT $limit
    ELSE
        SELECT * FROM problem WHERE owner = type::thing("account", $id)
    END;
ELSE
    IF $limit THEN
        SELECT * FROM problem WHERE owner = type::thing("account", $id) AND private = false LIMIT $limit
    ELSE
        SELECT * FROM problem WHERE owner = type::thing("account", $id) AND private = false
    END;
END;"#;

pub async fn list_for_account<M>(
    db: &Surreal<Client>,
    account_id: Option<String>,
    authed_id: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<M>>
where
    for<'de> M: Deserialize<'de>,
{
    let mut response = db
        .query(LIST_QUERY)
        .bind((
            "authed",
            authed_id.is_some() && account_id.is_some() && authed_id == account_id,
        ))
        .bind(("id", account_id))
        .bind(("limit", limit))
        .await?;

    Ok(response.take(0)?)
}
