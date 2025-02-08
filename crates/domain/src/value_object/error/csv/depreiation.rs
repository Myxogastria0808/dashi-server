use axum::http::StatusCode;
use thiserror::Error;

use crate::value_object::error::AppError;

#[derive(Debug, Error)]
pub enum DepreiationCsvError {
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<DepreiationCsvError> for AppError {
    fn from(error: DepreiationCsvError) -> Self {
        match error {
            DepreiationCsvError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
