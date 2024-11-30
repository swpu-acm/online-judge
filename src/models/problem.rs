use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::routes::problem::CreateProblem;

use super::UserRecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: Option<Thing>,

    pub title: String,
    pub description: String,
    pub input: Option<String>,
    pub output: Option<String>,
    pub samples: Vec<Sample>,
    pub hint: Option<String>,

    pub time_limit: u64,
    pub memory_limit: u64,
    pub test_cases: Vec<Sample>,

    pub creator: Thing,
    pub owner: Thing,
    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub private: bool,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<CreateProblem<'_>> for Problem {
    fn from(val: CreateProblem<'_>) -> Self {
        Problem {
            id: None,
            title: val.title.to_string(),
            description: val.description.to_string(),
            input: val.input,
            output: val.output,
            samples: val.samples,
            hint: val.hint,
            time_limit: val.time_limit,
            memory_limit: val.memory_limit,
            test_cases: val.test_cases,
            creator: ("account", val.id).into(),
            owner: val.owner.into(),
            categories: val.categories,
            tags: val.tags,
            private: val.private,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemDetail {
    pub id: String,

    pub title: String,
    pub description: String,
    pub input: Option<String>,
    pub output: Option<String>,
    pub samples: Vec<Sample>,
    pub hint: Option<String>,

    pub time_limit: u64,
    pub memory_limit: u64,
    pub test_cases: Vec<Sample>,

    pub creator: UserRecordId,
    pub owner: UserRecordId,
    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub private: bool,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<Problem> for ProblemDetail {
    fn from(value: Problem) -> Self {
        ProblemDetail {
            id: value.id.unwrap().id.to_string(),
            title: value.title,
            description: value.description,
            input: value.input,
            output: value.output,
            samples: value.samples,
            hint: value.hint,
            time_limit: value.time_limit,
            memory_limit: value.memory_limit,
            test_cases: value.test_cases,
            creator: value.creator.into(),
            owner: value.owner.into(),
            categories: value.categories,
            tags: value.tags,
            private: value.private,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
