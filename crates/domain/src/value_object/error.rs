use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub mod api;
pub mod connection;
pub mod critical_incident;
pub mod generate;
pub mod healthcheck;
pub mod register_item;

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
