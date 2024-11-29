use crate::models::UserRecordId;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category<'d> {
    pub id: Option<Thing>,
    pub owner: Thing,
    pub name: &'d str,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateCategory {
    pub name: String,
    pub owner: UserRecordId,
}
