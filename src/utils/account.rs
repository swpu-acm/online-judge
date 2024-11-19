use anyhow::Result;
use serde::Deserialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::models::account::{Account, Profile, Role};
use crate::models::UpdateAt;
use crate::routes::account::RegisterData;

pub async fn create(db: &Surreal<Client>, register: RegisterData) -> Result<Option<Account>> {
    let mut queried = db
        .query("SELECT * FROM account WHERE username = $username OR email = $email")
        .bind(("username", register.username.clone()))
        .bind(("email", register.email.clone()))
        .await?;
    let exist_account: Option<Account> = queried.take(0)?;
    if exist_account.is_some() {
        return Ok(None);
    }

    let account: Option<Account> = db
        .create("account")
        .content(Account {
            username: register.username,
            password: register.password,
            email: register.email,
            role: Role::User,
            created_at: chrono::Local::now().naive_local(),
            ..Default::default()
        })
        .await?;
    Ok(account)
}

pub async fn get_by_id<M>(db: &Surreal<Client>, id: &str) -> Result<Option<M>>
where
    for<'de> M: Deserialize<'de>,
{
    Ok(db.select(("account", id)).await?)
}

pub async fn merge_profile(
    db: &Surreal<Client>,
    id: &str,
    profile: Profile,
) -> Result<Option<Account>> {
    db.update::<Option<Account>>(("account", id))
        .merge(profile)
        .await?;
    Ok(db
        .update(("account", id))
        .merge(UpdateAt {
            updated_at: chrono::Local::now().naive_local(),
        })
        .await?)
}

pub async fn delete(db: &Surreal<Client>, id: &str) -> Result<()> {
    db.delete::<Option<Account>>(("account", id)).await?;
    Ok(())
}
