use crate::models::submission::Submission;
use anyhow::Result;
use eval_stack::compile::Language;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

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

pub async fn list_by_contest(db: &Surreal<Client>, contest_id: Thing) -> Result<Vec<Submission>> {
    Ok(db
        .query("SELECT * FROM submission WHERE contest_id = $contest_id")
        .bind(("contest_id", contest_id))
        .await?
        .take(0)?)
}

pub async fn list_within_contest(
    db: &Surreal<Client>,
    contest_id: Thing,
    user_id: Thing,
) -> Result<Vec<Submission>> {
    let submissions = list_by_contest(db, contest_id).await?;

    Ok(submissions
        .into_iter()
        .filter(|s| s.creator == user_id)
        .collect())
}

pub async fn list_by_problem(db: &Surreal<Client>, problem: Thing) -> Result<Vec<Submission>> {
    Ok(db
        .query("SELECT * FROM submission WHERE problem = &problem")
        .bind(("problem", problem))
        .await?
        .take(0)?)
}
