use std::vec;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: Option<Thing>,
    pub name: String,
    pub display_name: Option<String>,

    pub description: Option<String>,

    pub owners: Vec<Thing>,
    pub members: Vec<Thing>,
    pub creator: Option<Thing>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OrganizationData<'c> {
    pub name: &'c str,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateOrg {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateOrganization<'r> {
    pub id: &'r str,
    pub token: &'r str,

    pub org: OrganizationData<'r>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserOrganization {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,

    pub owners: Vec<String>,
    pub members: Vec<String>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChangeMember<'r> {
    pub id: &'r str,
    pub token: &'r str,
    pub members: Vec<String>,
}

impl From<CreateOrganization<'_>> for Organization {
    fn from(val: CreateOrganization) -> Self {
        Organization {
            id: None,
            name: val.org.name.to_string(),
            display_name: val.org.display_name,
            description: val.org.description,
            owners: vec![("account", val.id).into()],
            members: vec![],
            creator: Some(("account".to_string(), val.id.to_string()).into()),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<Organization> for UserOrganization {
    fn from(val: Organization) -> Self {
        UserOrganization {
            name: val.name,
            display_name: val.display_name,
            description: val.description,
            owners: val.owners.iter().map(|x| x.to_string()).collect(),
            members: val.members.iter().map(|x| x.to_string()).collect(),
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

impl From<OrganizationData<'_>> for UpdateOrg {
    fn from(val: OrganizationData) -> Self {
        UpdateOrg {
            name: val.name.to_string(),
            display_name: val.display_name,
            description: val.description,
        }
    }
}
