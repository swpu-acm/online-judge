use algohub_server::{
    models::{
        account::Register,
        contest::{AddProblems, ContestData, CreateContest, Mode, Visibility},
        problem::{CreateProblem, ProblemVisibility},
        response::{Empty, Response},
        submission::Submission,
        Credentials, OwnedCredentials, OwnedId, Token, UserRecordId,
    },
    routes::{index::init_db, problem::ProblemResponse, submission::CreateSubmission},
};
use anyhow::Result;
use rocket::local::asynchronous::Client;
pub mod utils;

#[rocket::async_test]
async fn test_submission() -> Result<()> {
    let db = init_db(utils::TEST_DB_ADDR)
        .await
        .expect("Failed to initialize database, shutting down");
    let rocket = algohub_server::rocket(db.clone()).await;

    let client = Client::tracked(rocket).await?;

    println!("Testing submission...");
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

    let id = data.id;
    let token = data.token;

    assert!(success);

    let response = client
        .post("/problem/create")
        .json(&CreateProblem {
            id: &id,
            token: &token,
            title: "Test Problem #1",
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
    println!("Created problem: {:?}", &problem_data);

    let response = client
        .post("/contest/create")
        .json(&CreateContest {
            auth: OwnedCredentials {
                id: id.clone(),
                token: token.clone(),
            },
            data: ContestData {
                name: "Test Contest".to_string(),
                mode: Mode::ICPC,
                visibility: Visibility::Public,
                description: "Test Description".to_string(),
                start_time: chrono::Local::now().naive_local(),
                end_time: chrono::Local::now().naive_local() + chrono::Duration::hours(1),
                owner: UserRecordId {
                    tb: "account".to_string(),
                    id: id.clone(),
                },
            },
        })
        .dispatch()
        .await;

    let Response {
        success,
        message: _,
        data,
    } = response.into_json().await.unwrap();

    assert!(success);
    let contest_data: OwnedId = data.unwrap();

    let response = client
        .post("/contest/problems/add")
        .json(&AddProblems {
            auth: OwnedCredentials {
                id: id.clone(),
                token: token.clone(),
            },
            contest_id: &contest_data.id,
            problem_ids: vec![&problem_data.id],
        })
        .dispatch()
        .await;

    response.into_json::<Response<Empty>>().await.unwrap();

    let mut new_submission: Vec<String> = Vec::new();

    for _ in 0..5 {
        let response = client
            .post(format!("/code/submit/{}", problem_data.id))
            .json(&CreateSubmission {
                auth: OwnedCredentials {
                    id: id.to_string(),
                    token: token.clone(),
                },

                code: "test".to_string(),
                lang: eval_stack::compile::Language::Rust,
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
        println!("Created submission: {}", data.id);
        new_submission.push(data.id);
    }

    let response = client
        .post(format!("/code/get/{}", new_submission[0]))
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
    let data: Submission = data.unwrap();

    assert!(success);
    println!("Get submissions by id: {:#?}", data);

    // let response = client
    //     .post(format!("/code/list/contest/{}", contest_data.id))
    //     .json(&Credentials {
    //         id: &id,
    //         token: &token,
    //     })
    //     .dispatch()
    //     .await;

    // assert_eq!(response.status().code, 200);

    // let Response {
    //     success,
    //     message: _,
    //     data,
    // } = response.into_json().await.unwrap();
    // let data: Vec<Submission> = data.unwrap();

    // assert!(success);
    // assert_eq!(data.len(), 5);

    // println!("Get submissions by contest: {:#?}", data);

    let response = client
        .post(format!("/code/list/user/{}", id))
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
    let data: Vec<Submission> = data.unwrap();

    assert!(success);
    assert_eq!(data.len(), 5);

    println!("Get submissions by user: {:#?}", data);

    // let response = client
    //     .post(format!("/code/list/contest/{}/{}", contest_data.id, id))
    //     .json(&Credentials {
    //         id: &id,
    //         token: &token,
    //     })
    //     .dispatch()
    //     .await;

    // assert_eq!(response.status().code, 200);

    // let Response {
    //     success,
    //     message: _,
    //     data,
    // } = response.into_json().await.unwrap();
    // let data: Vec<Submission> = data.unwrap();

    // assert!(success);
    // assert_eq!(data.len(), 5);
    // println!("Get submissions by user within a contest: {:#?}", data);

    let response = client
        .post(format!("/code/list/problem/{}", problem_data.id))
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
    let data: Vec<Submission> = data.unwrap();

    assert_eq!(data.len(), 5);

    assert!(success);
    println!("Get submissions by problem: {:#?}", data);

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
