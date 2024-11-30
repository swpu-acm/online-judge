use crate::models::submission::{Submission, UserSubmission};
use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub async fn create<'a>(
    db: &'a Surreal<Client>,
    id: &str,
    submission: UserSubmission<'a>,
) -> Result<Option<Submission>> {

    Ok(db
        .create("submission")
        .content(Submission {
            id: None,
            lang: submission.lang,
            code: submission.code,
            problem_id: submission.problem_id.to_string(),
            status: crate::models::submission::Status::InQueue,

            creator: ("account",id).into(),
            results: vec![],

            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}
