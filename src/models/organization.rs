use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: Option<Thing>,
    pub name: String,
    pub display_name: Option<String>,

    pub description: Option<String>,

    pub owners: Vec<Thing>,
    pub members: Vec<Thing>,

    pub creator: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OrganizationData<'c> {
    pub name: &'c str,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateOrganization<'r> {
    pub id: &'r str,
    pub token: &'r str,

    pub org: OrganizationData<'r>,
}
