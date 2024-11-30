use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::{OwnedCredentials, UserRecordId};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Internal,
    Private,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    ICPC,
    OI,
}

#[derive(Serialize, Deserialize)]
pub struct Contest {
    pub id: Option<Thing>,

    pub name: String,
    pub mode: Mode,
    pub visibility: Visibility,
    pub description: String,
    pub announcement: Option<String>,

    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub problems: Vec<Thing>,

    pub owner: Thing,
    pub creator: Thing,
    pub updaters: Vec<Thing>,
    pub participants: Vec<Thing>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct ContestData {
    pub name: String,
    pub mode: Mode,
    pub visibility: Visibility,
    pub description: String,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub owner: UserRecordId,
}

#[derive(Serialize, Deserialize)]
pub struct CreateContest {
    pub auth: OwnedCredentials,
    pub data: ContestData,
}

#[derive(Serialize, Deserialize)]
pub struct AddProblems<'a> {
    pub auth: OwnedCredentials,
    pub contest_id: &'a str,
    pub problem_ids: Vec<&'a str>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveProblem {
    pub auth: OwnedCredentials,
    pub contest_id: Thing,
    pub problem_id: Thing,
}
