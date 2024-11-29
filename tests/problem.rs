use algohub_server::{
    models::{
        problem::{Mode, ProblemDetail},
        response::{Empty, Response},
        OwnedCredentials, Token,
    },
    routes::{
        account::{RegisterData, RegisterResponse},
        problem::{CreateProblem, ListProblem, ProblemResponse},
    },
};
use anyhow::Result;
use rocket::local::asynchronous::Client;

#[rocket::async_test]
async fn test_problem() -> Result<()> {
    let rocket = algohub_server::rocket().await;

    let client = Client::tracked(rocket).await?;

    println!("Testing register...");
    let response = client
        .post("/account/create")
        .json(&RegisterData {
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
    let data: RegisterResponse = data.unwrap();

    let id = data.id.clone();
    let token = data.token.clone();

    assert!(success);
    println!("Registered account: {:?}", &data);

    for i in 0..10 {
        let response = client
            .post("/problem/create")
            .json(&CreateProblem {
                id: &id,
                token: &token,
                title: &format!("Test Problem #{}", i),
                description: "Test Description".to_string(),
                input: Some("Test Input".to_string()),
                output: Some("Test Output".to_string()),
                samples: vec![],
                hint: None,

                time_limit: 1000,
                memory_limit: 128,
                test_cases: vec![],
                categories: vec![],
                tags: vec![],
                mode: Mode::ICPC,
                private: true,
            })
            .dispatch()
            .await;

        assert_eq!(response.status().code, 200);

        let Response {
            success,
            message: _,
            data,
        } = response.into_json().await.unwrap();
        let data: ProblemResponse = data.unwrap();

        assert!(success);
        println!("Created problem: {:?}", &data);
    }

    let response = client
        .post("/problem/list")
        .json(&ListProblem {
            identity: Some(id.clone()),
            auth: Some(OwnedCredentials {
                id: id.clone(),
                token: token.clone(),
            }),
            limit: None,
        })
        .dispatch()
        .await;

    let Response {
        success,
        message: _,
        data,
    } = response
        .into_json::<Response<Vec<ProblemDetail>>>()
        .await
        .unwrap();
    let data = data.unwrap();
    assert!(success);
    assert_eq!(data.len(), 10);

    let response = client
        .post("/problem/list")
        .json(&ListProblem {
            identity: Some(id.clone()),
            auth: Some(OwnedCredentials {
                id: id.clone(),
                token: token.clone(),
            }),
            limit: Some(3),
        })
        .dispatch()
        .await;

    let Response {
        success,
        message: _,
        data,
    } = response
        .into_json::<Response<Vec<ProblemDetail>>>()
        .await
        .unwrap();
    let data = data.unwrap();
    assert!(success);
    assert_eq!(data.len(), 3);

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
