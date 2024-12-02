use std::path::PathBuf;

use rocket::{
    fs::{NamedFile, TempFile},
    response::Responder,
};
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

pub struct AssetFile(pub(crate) NamedFile);

impl<'r, 'o: 'r> Responder<'r, 'o> for AssetFile {
    fn respond_to(self, req: &rocket::Request) -> rocket::response::Result<'o> {
        rocket::Response::build_from(self.0.respond_to(req)?)
            .raw_header("Cache-control", "max-age=86400") //  24h (24*60*60)
            .ok()
    }
}
