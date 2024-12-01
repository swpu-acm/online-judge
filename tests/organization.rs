use std::path::Path;

use algohub_server::{
    models::{
        account::Register,
        organization::CreateOrganization,
        response::{Empty, Response},
        OwnedCredentials, Token,
    },
    routes::organization::{CreateOrgResponse, OrgData},
};
use anyhow::Result;
use rocket::local::asynchronous::Client;

#[rocket::async_test]
async fn test_organization() -> Result<()> {
    let rocket = algohub_server::rocket().await;

    let client = Client::tracked(rocket).await?;

    println!("Testing organization...");
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
        .post("/organization/create")
        .json(&OrgData {
            id: &id,
            token: &token,
            org: CreateOrganization {
                name: "test_organization".to_string(),
                display_name: None,
                description: None,
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
    let data: CreateOrgResponse = data.unwrap();

    assert!(success);
    println!("Created organization: {}", data.id);

    let response = client
        .post(format!("/organization/delete/{}", id))
        .json(&OrgData {
            id: &id,
            token: &token,
            org: CreateOrganization {
                name: "test_organization".to_string(),
                display_name: None,
                description: None,
            },
        })
        .dispatch()
        .await;

    response.into_json::<Response<Empty>>().await.unwrap();

    assert!(!Path::new("content").join(id.clone()).exists());

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
