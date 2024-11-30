use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, opt::PatchOp, sql::Thing, Surreal};

use crate::models::contest::{Contest, ContestData};

pub async fn create(
    db: &Surreal<Client>,
    creator_id: &str,
    contest: ContestData,
) -> Result<Option<Contest>> {
    Ok(db
        .create("contest")
        .content(Contest {
            id: None,
            name: contest.name.to_string(),
            mode: contest.mode,
            visibility: contest.visibility,
            description: contest.description,
            announcement: None,
            start_time: contest.start_time,
            end_time: contest.end_time,
            problems: vec![],
            owner: contest.owner.clone().into(),
            creator: ("account", creator_id).into(),
            updaters: vec![("account", creator_id).into()],
            participants: vec![],
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        })
        .await?)
}

pub async fn get(db: &Surreal<Client>, id: &str) -> Result<Option<Contest>> {
    Ok(db.select(("contest", id)).await?)
}

pub async fn list(db: &Surreal<Client>, id: Thing) -> Result<Vec<Contest>> {
    Ok(db
        .query("SELECT * FROM contest WHERE owner = $id")
        .bind(("id", id))
        .await?
        .take(0)?)
}

pub async fn add_problems(
    db: &Surreal<Client>,
    id: &str,
    problems: &[Thing],
) -> Result<Option<Contest>> {
    Ok(db
        .update(("contest", id))
        .patch(PatchOp::add("/problems", problems))
        .await?)
}

const REMOVE_PROBLEM: &str =
    "UPDATE contest SET problems -= type::thing(\"problem\", $problem) WHERE record::id(id) = $id";
pub async fn remove_problem(
    db: &Surreal<Client>,
    id: String,
    problem: Thing,
) -> Result<Option<Contest>> {
    Ok(db
        .query(REMOVE_PROBLEM)
        .bind(("id", id))
        .bind(("problem", problem))
        .await?
        .take(0)?)
}
