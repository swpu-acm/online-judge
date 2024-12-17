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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSolution {
    pub id: String,

    pub problem: String,
    pub title: String,
    pub content: String,
    pub creator: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<Solution> for UserSolution {
    fn from(value: Solution) -> Self {
        UserSolution {
            id: value.id.unwrap().id.to_string(),
            problem: value.problem.id.to_string(),
            creator: value.creator.id.to_string(),
            title: value.title,
            content: value.content,

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<CreateSolution<'_>> for Solution {
    fn from(val: CreateSolution<'_>) -> Self {
        Solution {
            id: None,
            title: val.data.title.to_string(),
            content: val.data.content.to_string(),
            problem: ("problem", val.data.problem).into(),
            creator: ("account", val.id).into(),

            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSolutions {
    pub problem: String,
}
