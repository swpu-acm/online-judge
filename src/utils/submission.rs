use anyhow::Result;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use crate::routes::submission::{CreateSubmission, SubmitData};

pub async fn submit(db: &Surreal<Client>, submission: CreateSubmission) -> Result<(Option<SubmitData>)> {
}