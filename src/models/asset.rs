use std::path::PathBuf;

use rocket::fs::TempFile;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::{Credentials, UserRecordId};

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub id: Option<Thing>,
    pub name: String,
    pub owner: Thing,
    pub path: PathBuf,
}

#[derive(FromForm)]
pub struct CreateAsset<'a> {
    pub auth: Credentials<'a>,
    pub owner: UserRecordId,
    pub file: TempFile<'a>,
}

#[derive(Serialize, Deserialize)]
pub struct UserContent {
    pub id: String,
}
