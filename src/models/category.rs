use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<Thing>,
    pub owner: Thing,
    pub name: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateCategory {
    pub name: String,
    pub group: String,
}
