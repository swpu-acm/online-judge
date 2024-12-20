use anyhow::Result;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::organization::{Organization, OrganizationData};

pub async fn create(
    db: &Surreal<Client>,
    id: &str,
    org: OrganizationData<'_>,
) -> Result<Option<Organization>> {
    Ok(db
        .create("organization")
        .content(Organization {
            id: None,
            name: org.name.to_string(),
            display_name: org.display_name,
            description: org.description,
            owners: vec![("account", id).into()],
            members: vec![],
            creator: id.to_string(),
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

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<Option<Organization>> {
    Ok(db.delete(("organization", id)).await?)
}
