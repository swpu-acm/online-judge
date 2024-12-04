use crate::models::submission::Status;
use crate::models::submission::Submission;
use anyhow::Result;
use eval_stack::compile::Language;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

pub async fn create(
    db: &Surreal<Client>,
    account_id: &str,
    problem: &str,
    code: String,
    lang: Language,
) -> Result<Option<Submission>> {
    Ok(db
        .create("submission")
        .content(Submission {
            id: None,
            lang,
            code,
            problem: ("problem", problem).into(),
            status: Status::InQueue,

            creator: ("account", account_id).into(),
            judge_details: vec![],
            judge_result: None,

            contest: None,

            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}

pub async fn get_by_id(db: &Surreal<Client>, id: &str) -> Result<Option<Submission>> {
    Ok(db.select(("submission", id)).await?)
}

pub async fn list_by_user(db: &Surreal<Client>, creator: Thing) -> Result<Vec<Submission>> {
    Ok(db
        .query("SELECT * FROM submission WHERE creator = $creator")
        .bind(("creator", creator))
        .await?
        .take(0)?)
}

pub async fn list_by_contest(db: &Surreal<Client>, contest: Thing) -> Result<Vec<Submission>> {
    Ok(db
        .query("SELECT * FROM submission WHERE contest = $contest")
        .bind(("contest", contest))
        .await?
        .take(0)?)
}

pub async fn list_within_contest(
    db: &Surreal<Client>,
    contest: Thing,
    creator: Thing,
) -> Result<Vec<Submission>> {
    Ok(db
        .query("SELECT * FROM submission WHERE contest = $contest AND creator = $creator")
        .bind(("contest", contest))
        .bind(("creator", creator))
        .await?
        .take(0)?)
}

pub async fn list_by_problem(db: &Surreal<Client>, problem: Thing) -> Result<Vec<Submission>> {
    Ok(db
        .query("SELECT * FROM submission WHERE problem = $problem")
        .bind(("problem", problem))
        .await?
        .take(0)?)
}
