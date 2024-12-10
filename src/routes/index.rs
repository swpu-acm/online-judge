use std::path::{Path, PathBuf};

use super::asset;
use super::category;
use super::contest;
use super::organization;
use super::problem;
use super::solution;
use super::submission;
use crate::{cors::CORS, routes::account};
use anyhow::Result;
use rocket::fs::NamedFile;
use surrealdb::engine::remote::ws::Client;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[get("/")]
async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("dist/index.html").await
}

#[get("/<file..>", rank = 1)]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("dist/").join(file)).await.ok()
}

pub async fn init_db(db_addr: &str) -> Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>(db_addr)
        .await
        .expect("Failed to connect to database");

    db.use_ns("main")
        .use_db("acm")
        .await
        .expect("Failed to use database");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to authenticate");

    Ok(db)
}

pub async fn rocket(db: Surreal<Client>) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, files])
        .mount("/account", account::routes())
        .mount("/asset", asset::routes())
        .mount("/problem", problem::routes())
        .mount("/org", organization::routes())
        .mount("/category", category::routes())
        .mount("/contest", contest::routes())
        .mount("/code", submission::routes())
        .mount("/solution", solution::routes())
        .manage(db)
}
