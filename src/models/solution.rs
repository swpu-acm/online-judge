use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    pub id: Option<Thing>,

    pub problem: Thing,
    pub creator: Thing,
    pub title: String,
    pub content: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SolutionData<'r> {
    pub title: &'r str,
    pub content: &'r str,
    pub problem: &'r str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateSolution<'r> {
    pub id: &'r str,
    pub token: &'r str,
    pub data: SolutionData<'r>,
}
