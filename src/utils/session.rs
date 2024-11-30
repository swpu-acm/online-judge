use anyhow::Result;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::models::account::{Account, Session};

use super::account;

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

pub async fn update(db: &Surreal<Client>, account: Thing) -> Result<Option<Session>> {
    let session: Option<Session> = db
        .upsert(("session", account.id.to_string()))
        .content(Session {
            id: None,
            account_id: account,
            token: uuid::Uuid::new_v4().to_string(),
        })
        .await?;
    Ok(session)
}

pub async fn authenticate(
    db: &Surreal<Client>,
    identity: &str,
    password: &str,
) -> Result<Option<Session>> {
    let account = account::get_by_identity::<Account>(db, identity).await?;
    if account.is_none() {
        return Ok(None);
    };
    let account = account.unwrap();
    if account.password == password {
        Ok(update(db, account.id.unwrap()).await?)
    } else {
        Ok(None)
    }
}
