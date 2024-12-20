#[macro_use]
extern crate rocket;

pub mod models;
pub mod utils {
    pub mod account;
    pub mod asset;
    pub mod category;
    pub mod contest;
    pub mod organization;
    pub mod problem;
    pub mod session;
    pub mod solution;
    pub mod submission;
}

pub mod routes {
    pub mod account;
    pub mod asset;
    pub mod category;
    pub mod contest;
    pub mod index;
    pub mod organization;

    pub mod problem;
    pub mod solution;
    pub mod submission;
}

pub mod cors;

use models::response::Response;
use rocket::serde::json::Json;
pub type Result<T> = std::result::Result<Json<Response<T>>, models::error::Error>;

pub use crate::routes::index::rocket;
