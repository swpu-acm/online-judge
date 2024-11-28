use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    id: Option<Thing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecordId {
    tb: String,
    id: String,
}

impl From<Thing> for UserRecordId {
    fn from(thing: Thing) -> Self {
        UserRecordId {
            tb: thing.tb,
            id: thing.id.to_string(),
        }
    }
}

impl Into<Thing> for UserRecordId {
    fn into(self) -> Thing {
        Thing::from((self.tb, self.id))
    }
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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Token<'r> {
    pub token: &'r str,
}
