use anyhow::Result;
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
            creator: todo!(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}
