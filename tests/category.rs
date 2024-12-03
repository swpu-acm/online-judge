mod utils;

use algohub_server::models::{
    account::Register,
    category::{Category, CategoryData, CreateCategory, ListCategories},
    response::{Empty, Response},
    OwnedCredentials, OwnedId, Token, UserRecordId,
};
use anyhow::Result;
use rocket::local::asynchronous::Client;
use utils::rocket;

#[rocket::async_test]
async fn test_category() -> Result<()> {
    let rocket = rocket().await;
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

    let mut new_category_ids: Vec<String> = Vec::new();

    for i in 0..10 {
        let response = client
            .post("/category/create")
            .json(&CreateCategory {
                id: &id,
                token: &token,
                data: CategoryData {
                    name: &format!("test_category_{}", i),
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
        let data: OwnedId = data.unwrap();

        assert!(success);
        println!("Created category: {}", data.id);
        new_category_ids.push(data.id);
    }

    let response = client
        .post("/category/list")
        .json(&ListCategories {
            owner: UserRecordId {
                tb: "account".to_string(),
                id: id.clone(),
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
    let data: Vec<Category> = data.unwrap();

    assert!(success);
    println!("Listed categories: {:#?}", data);

    for new_category_id in new_category_ids.iter().take(10) {
        let response = client
            .post(format!("/category/delete/{}", new_category_id))
            .json(&CreateCategory {
                id: &id,
                token: &token,
                data: CategoryData {
                    name: "test_category",
                    owner: UserRecordId {
                        tb: "account".to_string(),
                        id: id.clone(),
                    },
                },
            })
            .dispatch()
            .await;

        response.into_json::<Response<Empty>>().await.unwrap();
    }

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
