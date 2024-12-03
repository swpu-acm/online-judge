use eval_stack::{compile::Language, judge::JudgeResult};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    InQueue,
    Judging,
    Ready,
}

#[derive(Serialize, Deserialize)]
pub struct Submission {
    pub id: Option<Thing>,

    pub lang: Language,
    pub problem_id: String,
    pub code: String,
    pub status: Status,
    pub results: Vec<JudgeResult>,
    pub creator: Thing,
    pub contest_id: Option<Thing>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
