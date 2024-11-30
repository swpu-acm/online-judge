use std::path::{Path, PathBuf};

use super::contest;
use super::organization;
use super::problem;
use super::submission;
use crate::{cors::CORS, routes::account};
use anyhow::Result;
use rocket::fs::NamedFile;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
#[get("/")]
async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("dist/index.html").await
}

#[get("/assets/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("dist/").join(file)).await.ok()
}

pub async fn rocket() -> rocket::Rocket<rocket::Build> {
    let db = Surreal::new::<Ws>("127.0.0.1:5176")
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

    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, files])
        .mount("/account", account::routes())
        .mount("/problem", problem::routes())
        .mount("/org", organization::routes())
        .mount("/contest", contest::routes())
        .mount("/code", submission::routes())
        .manage(db)
}
