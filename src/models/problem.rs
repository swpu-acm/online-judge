use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::UserRecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub input: String,
    pub output: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TestCase {
    pub input: Thing,
    pub output: Thing,
}

impl From<UserTestCase<'_>> for TestCase {
    fn from(value: UserTestCase<'_>) -> Self {
        TestCase {
            input: Thing::from(("asset", value.input)),
            output: Thing::from(("asset", value.output)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemVisibility {
    ContestOnly,
    Public,
    Private,
    Internal,
}

#[derive(Clone, Serialize, Deserialize)]
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
    pub test_cases: Vec<TestCase>,

    pub creator: Thing,
    pub owner: Thing,
    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub visibility: ProblemVisibility,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTestCase<'r> {
    pub input: &'r str,
    pub output: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateProblem<'r> {
    pub id: &'r str,
    pub token: &'r str,

    pub title: &'r str,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    pub samples: Vec<Sample>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,

    pub owner: UserRecordId,
    pub time_limit: u64,
    pub memory_limit: u64,
    pub test_cases: Vec<UserTestCase<'r>>,

    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub visibility: ProblemVisibility,
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
            test_cases: val.test_cases.into_iter().map(Into::into).collect(),
            creator: ("account", val.id).into(),
            owner: val.owner.into(),
            categories: val.categories,
            tags: val.tags,
            visibility: val.visibility,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProblem {
    pub id: String,

    pub title: String,
    pub description: String,
    pub input: Option<String>,
    pub output: Option<String>,
    pub samples: Vec<Sample>,
    pub hint: Option<String>,

    pub time_limit: u64,
    pub memory_limit: u64,

    pub creator: String,
    pub owner: UserRecordId,
    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub visibility: ProblemVisibility,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<Problem> for UserProblem {
    fn from(value: Problem) -> Self {
        UserProblem {
            id: value.id.unwrap().id.to_string(),
            title: value.title,
            description: value.description,
            input: value.input,
            output: value.output,
            samples: value.samples,
            hint: value.hint,
            time_limit: value.time_limit,
            memory_limit: value.memory_limit,
            creator: value.creator.id.to_string(),
            owner: value.owner.into(),
            categories: value.categories,
            tags: value.tags,
            visibility: value.visibility,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
