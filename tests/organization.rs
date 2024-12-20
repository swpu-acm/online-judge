mod utils;

use std::path::Path;

use algohub_server::models::{
    account::Register,
    organization::{ChangeMember, CreateOrganization, OrganizationData, UserOrganization},
    response::{Empty, Response},
    Credentials, OwnedCredentials, OwnedId, Token,
};
use anyhow::Result;
use rocket::local::asynchronous::Client;
use utils::rocket;

#[rocket::async_test]
async fn test_organization() -> Result<()> {
    let rocket = rocket().await;

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
        .post("/org/create")
        .json(&CreateOrganization {
            id: &id,
            token: &token,
            org: OrganizationData {
                name: "test_organization",
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
    let org_data: OwnedId = data.unwrap();

    assert!(success);
    println!("Created organization: {}", org_data.id);

    let response = client
        .post(format!("/org/get/{}", org_data.id))
        .json(&Credentials {
            id: &id,
            token: &token,
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    let Response {
        success,
        message: _,
        data,
    } = response.into_json().await.unwrap();
    let data: UserOrganization = data.unwrap();

    assert!(success);
    println!("Get organization: {}", data.name);

    let response = client
        .post(format!("/org/add/{}", org_data.id))
        .json(&ChangeMember {
            id: &id,
            token: &token,
            members: vec!["k0nnyaku".to_string()],
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    response.into_json::<Response<Empty>>().await.unwrap();

    assert!(success);
    println!("add member: {}", "k0nnyaku");

    let response = client
        .post(format!("/org/remove/{}", org_data.id))
        .json(&ChangeMember {
            id: &id,
            token: &token,
            members: vec!["k0nnyaku".to_string()],
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    response.into_json::<Response<Empty>>().await.unwrap();

    assert!(success);
    println!("remove member: {}", "k0nnyaku");

    let response = client
        .post(format!("/org/update/{}", org_data.id))
        .json(&CreateOrganization {
            id: &id,
            token: &token,
            org: OrganizationData {
                name: "test_organization_update",
                display_name: None,
                description: None,
            },
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    assert!(success);
    println!("updated organization: {}", org_data.id);

    let response = client
        .post(format!("/org/delete/{}", org_data.id))
        .json(&Credentials {
            id: &id,
            token: &token,
        })
        .dispatch()
        .await;

    response.into_json::<Response<Empty>>().await.unwrap();

    assert!(!Path::new("content").join(org_data.id.clone()).exists());

    println!("Deleted organization: {}", org_data.id);

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
