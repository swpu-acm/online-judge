use rocket::form::{FromForm, FromFormField};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    id: Option<Thing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecordId {
    pub tb: String,
    pub id: String,
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for UserRecordId {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        if let Some((tb, id)) = field.value.split_once(':') {
            Ok(UserRecordId {
                tb: tb.to_string(),
                id: id.to_string(),
            })
        } else {
            Err(field.unexpected())?
        }
    }

    async fn from_data(field: rocket::form::DataField<'v, '_>) -> rocket::form::Result<'v, Self> {
        Err(field.unexpected())?
    }

    fn default() -> Option<Self> {
        None
    }
}

impl From<Thing> for UserRecordId {
    fn from(thing: Thing) -> Self {
        UserRecordId {
            tb: thing.tb,
            id: thing.id.to_string(),
        }
    }
}

impl From<UserRecordId> for Thing {
    fn from(value: UserRecordId) -> Self {
        Thing::from((value.tb, value.id))
    }
}

#[derive(Serialize)]
pub struct UpdateAt {
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(FromForm, Serialize, Deserialize)]
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
pub struct OwnedId {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Token<'r> {
    pub token: &'r str,
}
