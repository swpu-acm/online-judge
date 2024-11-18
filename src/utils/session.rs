use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::models::account::Session;

pub async fn create(db: &Surreal<Client>, account_id: Thing) -> Result<Option<Session>> {
    let session: Option<Session> = db
        .upsert(("session", account_id.id.to_string()))
        .content(Session {
            id: None,
            account_id,
            token: uuid::Uuid::new_v4().to_string(),
        })
        .await?;
    Ok(session)
}

pub async fn verify(db: &Surreal<Client>, account_id: &str, token: &str) -> bool {
    match db.select::<Option<Session>>(("session", account_id)).await {
        Ok(Some(session)) => session.token == token,
        _ => false,
    }
}
