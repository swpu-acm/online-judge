use rocket::{serde::json::Json, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

impl<T: ToString> From<T> for ErrorResponse {
    fn from(message: T) -> Self {
        Self {
            success: false,
            message: message.to_string(),
        }
    }
}

#[derive(Responder)]
pub enum Error {
    #[response(status = 500, content_type = "json")]
    ServerError(Json<ErrorResponse>),
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorResponse>),
    #[response(status = 401, content_type = "json")]
    Unauthorized(Json<ErrorResponse>),
    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorResponse>),
    #[response(status = 405, content_type = "json")]
    MethodNotAllowed(Json<ErrorResponse>),
    #[response(status = 409, content_type = "json")]
    Conflict(Json<ErrorResponse>),
    #[response(status = 403, content_type = "json")]
    Forbidden(Json<ErrorResponse>),
}

impl<T: ToString> From<T> for Error {
    fn from(e: T) -> Self {
        Error::ServerError(Json(e.into()))
    }
}
