use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub mod connection;

#[derive(Debug)]
pub struct AppError {
    pub status_code: StatusCode,
    pub code: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Json(json!({
            "status_code": format!("{}", self.status_code),
            "code": self.code,
            "message": self.message,
        }))
        .into_response()
    }
}
