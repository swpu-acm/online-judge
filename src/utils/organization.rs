use anyhow::Result;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::models::organization::{CreateOrganization, Organization};

pub async fn create(
    db: &Surreal<Client>,
    id: &str,
    org: CreateOrganization,
) -> Result<Option<Organization>> {
    let creator = Thing::from(("account", id));
    Ok(db
        .create("organization")
        .content(Organization {
            id: None,
            name: org.name,
            display_name: org.display_name,
            description: org.description,
            owner: vec![creator.clone()],
            member: vec![],
            creator,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}

pub async fn get_by_id<M>(db: &Surreal<Client>, id: &str) -> Result<Option<M>>
where
    for<'de> M: Deserialize<'de>,
{
    Ok(db.select(("organization", id)).await?)
}
