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
}

pub use crate::routes::index::rocket;
