use std::{fs::File, io::Read};

use algohub_server::{models::Credentials, routes::index::init_db};

pub struct Upload<'a> {
    pub auth: Credentials<'a>,
    pub owner_id: &'a str,
    pub file: File,
}

impl AsRef<[u8]> for Upload<'_> {
    fn as_ref(&self) -> &[u8] {
        let boundary = "boundary";
        let mut body = Vec::new();

        body.extend(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"auth[id]\"\r\n\r\n{}\r\n",
                self.auth.id
            )
            .as_bytes(),
        );

        body.extend(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"auth[token]\"\r\n\r\n{}\r\n",
                self.auth.token
            )
            .as_bytes(),
        );

        body.extend(
            format!(
                "--{boundary}\r\nContent-Disposition: form-data; name=\"owner\"\r\n\r\naccount:{}\r\n",
                self.owner_id
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

pub const TEST_DB_ADDR: &str = "localhost:27017";

pub async fn rocket() -> rocket::Rocket<rocket::Build> {
    algohub_server::rocket(
        init_db(TEST_DB_ADDR)
            .await
            .expect("Failed to initialize database, shutting down"),
    )
    .await
}
