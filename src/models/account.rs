use rocket::FromForm;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Role {
    SuperAdmin,
    Admin,
    #[default]
    User,
    Reserve,
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
