use super::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchItemError {
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
}

impl From<SearchItemError> for AppError {
    fn from(error: SearchItemError) -> Self {
        match error {
            SearchItemError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
        }
    }
}
