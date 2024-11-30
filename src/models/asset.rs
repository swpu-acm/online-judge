use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub id: Option<Thing>,
    pub name: String,
    pub path: PathBuf,
}
