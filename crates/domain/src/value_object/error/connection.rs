use super::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error(transparent)]
    DotEnvNotFountError(#[from] dotenvy::Error),
    #[error(transparent)]
    DotEnvVarError(#[from] std::env::VarError),
    #[error("Failed to get {0}")]
    DotEnvVarNotFountError(String),
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<ConnectionError> for AppError {
    fn from(error: ConnectionError) -> Self {
        match error {
            ConnectionError::DotEnvNotFountError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/dotenv-not-found".to_string(),
                message: e.to_string(),
            },
            ConnectionError::DotEnvVarError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/dotenv-var".to_string(),
                message: e.to_string(),
            },
            ConnectionError::DotEnvVarNotFountError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/dotenv-var-not-found".to_string(),
                message: format!("Failed to get {}", e),
            },
            ConnectionError::GraphDBError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/graphdb".to_string(),
                message: e.to_string(),
            },
            ConnectionError::MeiliSearchError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/meilisearch".to_string(),
                message: e.to_string(),
            },
            ConnectionError::RDBError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/rdb".to_string(),
                message: e.to_string(),
            },
        }
    }
}
