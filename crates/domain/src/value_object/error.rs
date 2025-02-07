use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

pub mod connection;
pub mod critical_incident;
pub mod delete_item;
pub mod generate;
pub mod healthcheck;
pub mod individual_item;
pub mod register_item;
pub mod search_item;
pub mod transfer_item;
pub mod update_item;

#[derive(Debug)]
pub struct AppError {
    pub status_code: StatusCode,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseError {
    pub code: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!(ResponseError {
                code: self.code,
                message: self.message,
            })),
        )
            .into_response()
    }
}
