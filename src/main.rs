use rocket::launch;

#[launch]
async fn rocket() -> _ {
    algohub_server::rocket().await
}
