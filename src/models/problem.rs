use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: Option<Thing>,
    pub sequence: String,

    pub title: String,
    pub content: String,
    pub input: String,
    pub output: String,
    pub samples: Vec<(String, String)>,

    pub time_limit: i32,
    pub memory_limit: i32,
    pub test_cases: Vec<(String, String)>,

    pub creator: Thing,
    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
