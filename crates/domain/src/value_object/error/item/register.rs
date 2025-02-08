use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterItemError {
    #[error("ItemNameEmptyError: Item name is empty.")]
    ItemNameEmptyError,
    #[error("LabelNotFoundError: Label not found.")]
    LabelNotFoundError,
    #[error("VisibleIdExisInItemTabletError: VisibleId already exists in Item Table.")]
    VisibleIdExistInItemTableError,
    #[error("VisibleIdConflictnItemTableError: Conflict VisibleId in Item Table.")]
    VisibleIdConflictInItemTableError,
    #[error("VisibleIdExistInMeiliSerachError: VisibleId already exists in MeiliSerach.")]
    VisibleIdExistInMeiliSerachError,
    #[error("VisibleIdConflictInMeiliSerachError: Conflict VisibleId in MeiliSerach.")]
    VisibleIdConflictInMeiliSerachError,
    #[error("ParentVisibleIdNotFoundInItemTableError: Parent VisibleId not found in Item Table.")]
    ParentVisibleIdNotFoundInItemTableError,
    #[error(
        "ParentVisibleIdNotFoundInMeiliSearchError: Parent VisibleId not found in MeiliSearch."
    )]
    ParentVisibleIdNotFoundInMeiliSearchError,
    #[error("ColorPatternExistInItemTableError: Color already exists in Item Table.")]
    ColorPatternExistInItemTableError,
    #[error("ColorPatternConflictInItemTableError: Conflict Color in Item Table.")]
    ColorPatternConflictInItemTableError,
    #[error("ColorPatternExistInMeiliSearchError: Color already exists in MeiliSearch.")]
    ColorPatternExistInMeiliSearcheError,
    #[error("ColorPatternConflictInMeiliSearchError: Conflict Color in Item MeiliSearch.")]
    ColorPatternConflictInMeiliSearchError,
    #[error("RegisteredItemNotFoundError: Registered item not found.")]
    RegisteredItemNotFoundError,
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<RegisterItemError> for AppError {
    fn from(error: RegisterItemError) -> Self {
        match error {
            RegisterItemError::ItemNameEmptyError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/item-name-empty".to_string(),
                message: "ItemNameEmptyError: Item name is empty.".to_string(),
            },
            RegisterItemError::LabelNotFoundError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/label-not-found".to_string(),
                message: "LabelNotFoundError: Label not found.".to_string(),
            },
            RegisterItemError::VisibleIdExistInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/visible-id-exist-in-item-table".to_string(),
                message: "VisibleIdExisInItemTabletError: VisibleId already exists in Item Table."
                    .to_string(),
            },
            RegisterItemError::VisibleIdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/conflict-in-item-table".to_string(),
                message: "VisibleIdConflictnItemTableError: Conflict VisibleId in Item Table."
                    .to_string(),
            },
            RegisterItemError::VisibleIdExistInMeiliSerachError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/visible-id-exist-in-meilisearch".to_string(),
                message:
                    "VisibleIdExistInMeiliSerachError: VisibleId already exists in MeiliSerach."
                        .to_string(),
            },
            RegisterItemError::VisibleIdConflictInMeiliSerachError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/conflict-in-meilisearch".to_string(),
                message: "VisibleIdConflictInMeiliSerachError: Conflict VisibleId in MeiliSerach."
                    .to_string(),
            },
            RegisterItemError::ParentVisibleIdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/parent-visible-id-not-found".to_string(),
                message: "ParentVisibleIdNotFoundInItemTableError: Parent VisibleId not found in Item Table."
                    .to_string(),
            },
            RegisterItemError::ParentVisibleIdNotFoundInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/parent-visible-id-not-found".to_string(),
                message: "ParentVisibleIdNotFoundInMeiliSearchError: Parent VisibleId not found in MeiliSerch."
                    .to_string(),
            },
            RegisterItemError::ColorPatternExistInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/color-pattern-exist-in-item-table".to_string(),
                message: "ColorPatternExistInItemTableError: Color already exists in Item Table."
                    .to_string(),
            },
            RegisterItemError::ColorPatternConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/color-pattern-conflict-in-item-table".to_string(),
                message: "ColorPatternConflictInItemTableError: Conflict Color in Item Table."
                    .to_string(),
            },
            RegisterItemError::ColorPatternExistInMeiliSearcheError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/color-pattern-exist-in-meilisearch".to_string(),
                message:
                    "ColorPatternExistInMeiliSearchError: Color already exists in MeiliSaerch."
                        .to_string(),
            },
            RegisterItemError::ColorPatternConflictInMeiliSearchError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/color-pattern-conflict-in-meilisearch".to_string(),
                message: "ColorPatternConflictInMeiliSearchError: Conflict Color in MeiliSearch."
                    .to_string(),
            },
            RegisterItemError::RegisteredItemNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/registered-item-not-found".to_string(),
                message: "RegisteredItemNotFoundError: Registered item not found.".to_string(),
            },
            RegisterItemError::GraphDBError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/graphdb".to_string(),
                message: e.to_string(),
            },
            RegisterItemError::MeiliSearchError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/meilisearch".to_string(),
                message: e.to_string(),
            },
            RegisterItemError::RDBError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/rdb".to_string(),
                message: e.to_string(),
            },
        }
    }
}
