mod utils;

use std::path::Path;

use algohub_server::{
    models::{
        account::Register,
        problem::{CreateProblem, ProblemVisibility},
        response::{Empty, Response},
        solution::{CreateSolution, ListSolutions, Solution, SolutionData, UserSolution},
        Credentials, OwnedCredentials, OwnedId, Token, UserRecordId,
    },
    routes::problem::ProblemResponse,
};
use anyhow::Result;
use rocket::local::asynchronous::Client;
use utils::rocket;

#[rocket::async_test]
async fn test_solution() -> Result<()> {
    let rocket = rocket().await;
    let client = Client::tracked(rocket).await?;

    println!("Testing solution");

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
        .post("/problem/create")
        .json(&CreateProblem {
            id: &id,
            token: &token,
            title: "Test Problem",
            description: "Test Description".to_string(),
            input: Some("Test Input".to_string()),
            output: Some("Test Output".to_string()),
            samples: vec![],
            hint: None,
            owner: UserRecordId {
                tb: "account".to_string(),
                id: id.clone(),
            },
            time_limit: 1000,
            memory_limit: 128,
            test_cases: vec![],
            categories: vec![],
            tags: vec![],
            visibility: ProblemVisibility::Public,
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    let Response {
        success,
        message: _,
        data,
    } = response.into_json().await.unwrap();
    let problem_data: ProblemResponse = data.unwrap();

    assert!(success);

    let response = client
        .post("/solution/create")
        .json(&CreateSolution {
            id: &id,
            token: &token,
            data: SolutionData {
                title: "test",
                content: "test",
                problem: &problem_data.id,
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
    let solution_data: OwnedId = data.unwrap();

    assert!(success);
    println!("Created solution: {}", solution_data.id);

    let response = client
        .post(format!("/solution/update/{}", solution_data.id))
        .json(&CreateSolution {
            id: &id,
            token: &token,
            data: SolutionData {
                title: "test2",
                content: "test2",
                problem: &problem_data.id,
            },
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    let Response {
        success,
        message: _,
        data: _,
    } = response.into_json::<Response<Empty>>().await.unwrap();

    assert!(success);

    println!("Update solution: {}", solution_data.id);

    let response = client
        .post(format!("/solution/get/{}", solution_data.id))
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
    let list_solution_data: UserSolution = data.unwrap();

    assert!(success);
    println!("get solution: {:#?}", list_solution_data);

    let response = client
        .post("/solution/list")
        .json(&ListSolutions {
            problem: problem_data.id.to_string(),
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    let Response {
        success,
        message: _,
        data,
    } = response.into_json().await.unwrap();
    let list_solution_data: Vec<Solution> = data.unwrap();

    assert!(success);
    println!("list solution: {:#?}", list_solution_data);

    let response = client
        .post(format!("/solution/delete/{}", solution_data.id))
        .json(&Credentials {
            id: &id,
            token: &token,
        })
        .dispatch()
        .await;

    response.into_json::<Response<Empty>>().await.unwrap();

    assert!(!Path::new("content").join(solution_data.id.clone()).exists());

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
