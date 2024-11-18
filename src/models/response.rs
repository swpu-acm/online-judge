use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Empty;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<D> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<D>,
}
