use rocket::{form::FromFormField, FromForm};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    SuperAdmin = 0,
    Admin = 1,
    #[default]
    User = 2,
    Reserve = 3,
    Inactive = 4,
}

impl TryFrom<i8> for Role {
    type Error = anyhow::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Role::SuperAdmin),
            1 => Ok(Role::Admin),
            2 => Ok(Role::User),
            3 => Ok(Role::Reserve),
            4 => Ok(Role::Inactive),
            _ => Err(anyhow::anyhow!("Invalid role: {}", value)),
        }
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for Role {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        let value = field.value.parse::<i8>()?;
        Ok(Role::try_from(value).map_err(|_| field.unexpected())?)
    }

    fn default() -> Option<Self> {
        Some(Self::User)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,
    pub email: String,
    pub avatar: Option<String>,
    pub signature: Option<String>,
    pub links: Option<Vec<String>>,

    pub nickname: Option<String>,
    pub sex: Option<bool>,
    pub birthday: Option<chrono::NaiveDateTime>,

    pub name: Option<String>,
    pub student_id: Option<String>,
    pub school: Option<String>,
    pub college: Option<String>,
    pub major: Option<String>,

    pub rating: i8,
    pub role: Role,
    pub active: bool,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(FromForm, Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub student_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub college: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Option<Thing>,
    pub account_id: Thing,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    id: Option<Thing>,
}
