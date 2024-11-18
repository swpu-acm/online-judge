use rocket::launch;

#[launch]
async fn rocket() -> _ {
    online_judge::rocket().await
}
