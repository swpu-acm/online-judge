use algohub_server::{
    models::{
        account::Register,
        category::CreateCategory,
        response::{Empty, Response},
        OwnedCredentials, Token, UserRecordId,
    },
    routes::category::{CategoryData, CreateCatResponse},
};
use anyhow::Result;
use rocket::local::asynchronous::Client;

#[rocket::async_test]
async fn test_category() -> Result<()> {
    let rocket = algohub_server::rocket().await;
    let client = Client::tracked(rocket).await?;

    println!("Testing category...");
    let response = client
        .post("/account/create")
        .json(&Register {
            username: "fu050409".to_string(),
            password: "password".to_string(),
            email: "email@example.com".to_string(),
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    let Response {
        success,
        message: _,
        data,
    } = response.into_json().await.unwrap();
    let data: OwnedCredentials = data.unwrap();

    let id = data.id.clone();
    let token = data.token.clone();

    assert!(success);

    let response = client
        .post("/category/create")
        .json(&CategoryData {
            id: &id,
            token: &token,
            data: CreateCategory {
                name: "test_category".to_string(),
                owner: UserRecordId {
                    tb: "account".to_string(),
                    id: id.clone(),
                },
            },
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    let Response {
        success,
        message: _,
        data,
    } = response.into_json().await.unwrap();
    let data: CreateCatResponse = data.unwrap();

    assert!(success);
    println!("Created category: {}", data.id);

    let response = client
        .post(format!("/category/delete/{}", data.id))
        .json(&CategoryData {
            id: &id,
            token: &token,
            data: CreateCategory {
                name: "test_category".to_string(),
                owner: UserRecordId {
                    tb: "account".to_string(),
                    id: id.clone(),
                },
            },
        })
        .dispatch()
        .await;

    response.into_json::<Response<Empty>>().await.unwrap();

    client
        .post(format!("/account/delete/{}", id))
        .json(&Token { token: &token })
        .dispatch()
        .await
        .into_json::<Response<Empty>>()
        .await
        .unwrap();
    Ok(())
}