use crate::models::UserRecordId;
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
pub struct CategoryData<'c> {
    pub name: &'c str,
    pub owner: UserRecordId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateCategory<'r> {
    pub id: &'r str,
    pub token: &'r str,

    pub data: CategoryData<'r>,
}

#[derive(Serialize, Deserialize)]
pub struct ListCategories {
    pub owner: UserRecordId,
}
