use serde::Serialize;

pub mod account;
pub mod error;
pub mod problem;
pub mod response;

#[derive(Serialize)]
pub struct UpdateAt {
    pub updated_at: chrono::NaiveDateTime,
}
