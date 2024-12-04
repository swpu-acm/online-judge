pub mod utils;

use std::{fs::File, path::Path};

use algohub_server::{
    models::{
        account::Register,
        asset::UserContent,
        problem::{CreateProblem, ProblemVisibility, UserProblem, UserTestCase},
        response::{Empty, Response},
        Credentials, OwnedCredentials, Token, UserRecordId,
    },
    routes::{
        index::init_db,
        problem::{ListProblem, ProblemResponse},
    },
};
use anyhow::Result;
use rocket::{http::ContentType, local::asynchronous::Client};
use utils::Upload;

#[rocket::async_test]
async fn test_problem() -> Result<()> {
    let db = init_db(utils::TEST_DB_ADDR)
        .await
        .expect("Failed to initialize database, shutting down");
    let rocket = algohub_server::rocket(db.clone()).await;

    let client = Client::tracked(rocket).await?;

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
                owner: UserRecordId {
                    tb: "account".to_string(),
                    id: id.clone(),
                },
                contest: None,
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
        .into_json::<Response<Vec<UserProblem>>>()
        .await
        .unwrap();
    let problems = data.unwrap();
    assert!(success);
    assert_eq!(problems.len(), 10);

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
        .into_json::<Response<Vec<UserProblem>>>()
        .await
        .unwrap();
    let data = data.unwrap();
    assert!(success);
    assert_eq!(data.len(), 3);

    for problem in problems {
        let response = client
            .delete(format!("/problem/delete/{}", problem.id))
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
            data: _,
        } = response.into_json::<Response<Empty>>().await.unwrap();
        assert!(success);
    }

    let input = client
        .put("/asset/upload")
        .header(ContentType::new("multipart", "form-data").with_params(("boundary", "boundary")))
        .body(Upload {
            auth: Credentials {
                id: &id,
                token: &token,
            },
            owner_id: &id,
            file: File::open("tests/1.in")?,
        })
        .dispatch()
        .await;
    let Response {
        success,
        message: _,
        data,
    } = input.into_json::<Response<UserContent>>().await.unwrap();
    assert!(success);
    let input = data.unwrap();

    let output = client
        .put("/asset/upload")
        .header(ContentType::new("multipart", "form-data").with_params(("boundary", "boundary")))
        .body(Upload {
            auth: Credentials {
                id: &id,
                token: &token,
            },
            owner_id: &id,
            file: File::open("tests/1.out")?,
        })
        .dispatch()
        .await;
    let Response {
        success,
        message: _,
        data,
    } = output.into_json::<Response<UserContent>>().await.unwrap();
    assert!(success);
    let output = data.unwrap();

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
            contest: None,
            time_limit: 1000,
            memory_limit: 128,
            test_cases: vec![UserTestCase {
                input: &input.id,
                output: &output.id,
            }],
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
    let data: ProblemResponse = data.unwrap();
    assert!(success);

    let test_cases = algohub_server::utils::problem::get_test_cases_by_id(&db, &data.id).await?;
    for test_case in &test_cases {
        assert!(Path::new(&test_case.input).exists());
        assert!(Path::new(&test_case.input).exists());
    }

    let response = client
        .delete(format!("/problem/delete/{}", data.id))
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
        data: _,
    } = response.into_json::<Response<Empty>>().await.unwrap();
    assert!(success);

    for test_case in test_cases {
        assert!(!Path::new(&test_case.input).exists());
        assert!(!Path::new(&test_case.input).exists());
    }
    assert!(
        algohub_server::utils::problem::get_test_cases_by_id(&db, &data.id)
            .await?
            .is_empty()
    );

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
