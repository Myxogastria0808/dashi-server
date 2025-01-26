use super::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HealthCheckError {
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    ConnectionError(#[from] crate::value_object::error::connection::ConnectionError),
}

impl From<HealthCheckError> for AppError {
    fn from(error: HealthCheckError) -> Self {
        match error {
            HealthCheckError::GraphDBError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/graphdb".to_string(),
                message: e.to_string(),
            },
            HealthCheckError::MeiliSearchError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/meilisearch".to_string(),
                message: e.to_string(),
            },
            HealthCheckError::RDBError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/rdb".to_string(),
                message: e.to_string(),
            },
            HealthCheckError::ConnectionError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/connection".to_string(),
                message: e.to_string(),
            },
        }
    }
}
