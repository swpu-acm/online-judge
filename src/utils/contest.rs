use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::models::contest::{Contest, ContestData, ContestProblem};

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

pub async fn list_all(db: &Surreal<Client>) -> Result<Vec<Contest>> {
    Ok(db.query("SELECT * FROM contest").await?.take(0)?)
}

pub async fn list_by_owner(db: &Surreal<Client>, id: &str) -> Result<Vec<Contest>> {
    Ok(db
        .query("SELECT * FROM contest WHERE record::id(owner) = $id")
        .bind(("id", id.to_string()))
        .await?
        .take(0)?)
}

const ADD_PROBLEM: &str = r#"
UPDATE type::thing("contest", $id)
SET problems = array::union(problems, $problems);
"#;
pub async fn add_problems(
    db: &Surreal<Client>,
    id: String,
    problems: Vec<Thing>,
) -> Result<Option<Contest>> {
    Ok(db
        .query(ADD_PROBLEM)
        .bind(("id", id))
        .bind(("problems", problems))
        .await?
        .take(0)?)
}

const LIST_PROBLEMS: &str = r#"
SELECT title, record::id(id) AS id, count(
    SELECT VALUE true
    FROM submission WHERE record::id(creator) == $account_id AND problem == $parent.id
    AND judge_result.status.type == "accepted"
) > 0 AS solved,
count(
    SELECT record::id(creator)
    FROM submission WHERE record::id(creator) == $account_id AND problem == $parent.id
) AS submittedCount,
count(
    SELECT record::id(creator)
    FROM submission WHERE record::id(creator) == $account_id AND problem == $parent.id
    AND judge_result.status.type == "accepted"
) AS acceptedCount
FROM type::thing("contest", $id).problems;
"#;
pub async fn list_problems(
    db: &Surreal<Client>,
    id: &str,
    account_id: &str,
) -> Result<Vec<ContestProblem>> {
    Ok(db
        .query(LIST_PROBLEMS)
        .bind(("id", id.to_string()))
        .bind(("account_id", account_id.to_string()))
        .await?
        .take(0)?)
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
