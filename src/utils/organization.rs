use anyhow::Result;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::organization::{CreateOrganization, Organization};

pub async fn create(db: &Surreal<Client>, org: CreateOrganization) -> Result<Option<Organization>> {
    Ok(db
        .create("organization")
        .content(Organization {
            id: None,
            name: org.name,
            display_name: org.display_name,
            description: org.description,
            // should be creator only
            owner: vec![],
            member: vec![],
            creator: org.creator,
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