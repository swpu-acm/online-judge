use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::routes::problem::ProblemData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    ICPC,
    OI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: Option<Thing>,

    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    pub samples: Vec<Sample>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,

    pub time_limit: i32,
    pub memory_limit: i32,
    pub test_cases: Vec<Sample>,

    pub creator: Thing,
    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub mode: Mode,
    pub private: bool,

    #[serde(skip)]
    pub created_at: chrono::NaiveDateTime,
    #[serde(skip)]
    pub updated_at: chrono::NaiveDateTime,
}

impl From<ProblemData<'_>> for Problem {
    fn from(val: ProblemData<'_>) -> Self {
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
            categories: val.categories,
            tags: val.tags,
            mode: val.mode,
            private: val.private,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}
