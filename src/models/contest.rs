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

#[derive(Serialize, Deserialize)]
pub struct UserContest {
    pub id: String,

    pub name: String,
    pub mode: Mode,
    pub visibility: Visibility,
    pub description: String,
    pub announcement: Option<String>,

    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,

    pub owner: UserRecordId,
    pub creator: String,
    pub updaters: Vec<String>,
    pub participants: Vec<String>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<Contest> for UserContest {
    fn from(value: Contest) -> Self {
        UserContest {
            id: value.id.unwrap().id.to_string(),
            name: value.name,
            mode: value.mode,
            visibility: value.visibility,
            description: value.description,
            announcement: value.announcement,
            start_time: value.start_time,
            end_time: value.end_time,
            owner: value.owner.into(),
            creator: value.creator.to_string(),
            updaters: value.updaters.iter().map(|x| x.id.to_string()).collect(),
            participants: value
                .participants
                .iter()
                .map(|x| x.id.to_string())
                .collect(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
