use anyhow::Result;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::models::organization::{CreateOrganization, Organization, OrganizationData, UpdateOrg};

pub async fn create(
    db: &Surreal<Client>,
    org: CreateOrganization<'_>,
) -> Result<Option<Organization>> {
    Ok(db
        .create("organization")
        .content(Into::<Organization>::into(org))
        .await?)
}

pub async fn get_by_id<M>(db: &Surreal<Client>, id: &str) -> Result<Option<M>>
where
    for<'de> M: Deserialize<'de>,
{
    Ok(db.select(("organization", id)).await?)
}

pub async fn update(
    db: &Surreal<Client>,
    id: &str,
    org: OrganizationData<'_>,
) -> Result<Option<Organization>> {
    Ok(db
        .update(("organization", id))
        .merge(Into::<UpdateOrg>::into(org))
        .await?)
}

const ADD_MEMBERS_QUERY: &str = r#"
UPDATE type::thing("organization", $id)
    SET members = array::union(members, $new_members)
"#;
pub async fn add(
    db: &Surreal<Client>,
    id: &str,
    member: Vec<&str>,
) -> Result<Option<Organization>> {
    let members_to_add: Vec<Thing> = member
        .into_iter()
        .map(|id| ("account", id).into())
        .collect();

    Ok(db
        .query(ADD_MEMBERS_QUERY)
        .bind(("id", id.to_string()))
        .bind(("new_members", members_to_add))
        .await?
        .take(0)?)
}

const REMOVE_MEMBERS_QUERY: &str = r#"
UPDATE type::thing("organization", $id)
    SET members = array::complement(members, $new_members)
"#;
pub async fn remove(
    db: &Surreal<Client>,
    id: &str,
    member: Vec<&str>,
) -> Result<Option<Organization>> {
    let members_to_remove: Vec<Thing> = member
        .into_iter()
        .map(|id| ("account", id).into())
        .collect();

    Ok(db
        .query(REMOVE_MEMBERS_QUERY)
        .bind(("id", id.to_string()))
        .bind(("new_members", members_to_remove))
        .await?
        .take(0)?)
}

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<Option<Organization>> {
    Ok(db.delete(("organization", id)).await?)
}
