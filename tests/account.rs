use std::{fs::File, io::Read, path::Path};

use algohub_server::{
    models::{
        account::Profile,
        response::{Empty, Response},
        Token,
    },
    routes::account::{ProfileData, RegisterData, RegisterResponse, UploadResponse},
};
use anyhow::Result;
use rocket::{http::ContentType, local::asynchronous::Client};

pub struct Upload {
    pub id: String,
    pub token: String,
    file: File,
}
impl AsRef<[u8]> for Upload {
    fn as_ref(&self) -> &[u8] {
        let boundary = "boundary";
        let mut body = Vec::new();

        body.extend(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"id\"\r\n\r\n{}\r\n",
                self.id
            )
            .as_bytes(),
        );

        body.extend(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"token\"\r\n\r\n{}\r\n",
                self.token
            )
            .as_bytes(),
        );

        body.extend(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.png\"\r\nContent-Type: image/png\r\n\r\n",
            )
            .as_bytes(),
        );

        let mut file_content = Vec::new();
        let mut file_clone = self.file.try_clone().expect("Failed to clone file");
        file_clone
            .read_to_end(&mut file_content)
            .expect("Failed to read file");
        body.extend(file_content);

        body.extend(format!("\r\n--{boundary}--\r\n").as_bytes());

        body.leak()
    }
}

#[rocket::async_test]
async fn test_register() -> Result<()> {
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

    let response = client
        .post("/account/profile")
        .json(&ProfileData {
            id: &data.id,
            token: &data.token,
            profile: Profile {
                email: None,
                username: None,
                avatar: None,
                signature: None,
                links: None,
                nickname: None,
                name: Some("苏向夜".into()),
                sex: None,
                birthday: None,
                student_id: None,
                school: None,
                college: None,
                major: None,
                rating: None,
                active: None,
                role: None,
            },
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    let Response {
        success,
        message: _,
        data: empty_data,
    } = response.into_json::<Response<Empty>>().await.unwrap();

    assert!(success);
    assert!(empty_data.is_none());

    let response = client
        .get(format!("/account/profile/{}", &id))
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    let Response {
        success,
        message: _,
        data,
    } = response.into_json().await.unwrap();
    let data: Profile = data.unwrap();

    assert!(success);
    assert_eq!(data.name, Some("苏向夜".into()));

    let response = client
        .put("/account/content/upload")
        .header(ContentType::new("multipart", "form-data").with_params(("boundary", "boundary")))
        .body(Upload {
            id: id.clone(),
            token: token.clone(),
            file: File::open("tests/test.png")?,
        })
        .dispatch()
        .await;

    assert_eq!(response.status().code, 200);

    let Response {
        success,
        message: _,
        data,
    } = response.into_json().await.unwrap();
    let data: UploadResponse = data.unwrap();

    assert!(success);
    assert!(data.uri.starts_with("/account/content/"));

    let response = client
        .post(format!("/account/delete/{}", id))
        .json(&Token { token: &token })
        .dispatch()
        .await;

    response.into_json::<Response<Empty>>().await.unwrap();

    assert!(!Path::new("content").join(id).exists());

    Ok(())
}
