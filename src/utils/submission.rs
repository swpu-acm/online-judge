use crate::models::submission::Submission;
use anyhow::Result;
use eval_stack::compile::Language;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub async fn create(
    db: &Surreal<Client>,
    account_id: &str,
    problem_id: &str,
    code: String,
    lang: Language,
) -> Result<Option<Submission>> {
    Ok(db
        .create("submission")
        .content(Submission {
            id: None,
            lang,
            code,
            problem_id: problem_id.to_string(),
            status: crate::models::submission::Status::InQueue,

            creator: ("account", account_id).into(),
            results: vec![],

            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}

pub async fn get_by_id(db: &Surreal<Client>, id: &str) -> Result<Option<Submission>> {
    Ok(db.select(("submission", id)).await?)
}
