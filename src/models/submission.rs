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

#[derive(Serialize, Deserialize, Debug)]
pub struct Submission {
    pub id: Option<Thing>,

    pub lang: Language,
    pub problem: Thing,
    pub code: String,
    pub status: Status,
    pub judge_details: Vec<JudgeResult>,
    pub judge_result: Option<JudgeResult>,
    pub creator: Thing,
    pub contest: Option<Thing>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
