#[macro_use]
extern crate rocket;

pub mod models;
pub mod utils {
    pub mod account;
    pub mod problem;
    pub mod session;
}

pub mod routes {
    pub mod account;
    pub mod index;
    pub mod problem;
}

pub mod cors;

use models::response::Response;
use rocket::serde::json::Json;
pub type Result<T> = std::result::Result<Json<Response<T>>, models::error::Error>;

pub use crate::routes::index::rocket;
