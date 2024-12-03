use algohub_server::routes::index::init_db;
use rocket::launch;

#[launch]
async fn rocket() -> _ {
    algohub_server::rocket(
        init_db("localhost:5177")
            .await
            .expect("Failed to initialize database, shutting down..."),
    )
    .await
}
