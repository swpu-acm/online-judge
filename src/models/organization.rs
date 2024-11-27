use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: Option<Thing>,
    pub name: String,
    pub display_name: Option<String>,

    pub description: Option<String>,

    pub owner: Vec<Thing>,
    pub member: Vec<Thing>,

    pub creator: Thing,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateOrganization {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
}
