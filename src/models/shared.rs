use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    id: Option<Thing>,
}

#[derive(Serialize)]
pub struct UpdateAt {
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials<'c> {
    pub id: &'c str,
    pub token: &'c str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OwnedCredentials {
    pub id: String,
    pub token: String,
}
