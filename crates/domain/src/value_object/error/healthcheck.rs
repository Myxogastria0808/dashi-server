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
            HealthCheckError::ConnectionError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/connection".to_string(),
                message: e.to_string(),
            },
            HealthCheckError::GraphDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/graphdb".to_string(),
                message: "GraphDBError: GraphDB trouble is occurred.".to_string(),
            },
            HealthCheckError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            HealthCheckError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
