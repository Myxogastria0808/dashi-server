use super::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferItemError {
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<TransferItemError> for AppError {
    fn from(error: TransferItemError) -> Self {
        match error {
            TransferItemError::GraphDBError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/graphdb".to_string(),
                message: e.to_string(),
            },
            TransferItemError::RDBError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/rdb".to_string(),
                message: e.to_string(),
            },
        }
    }
}
