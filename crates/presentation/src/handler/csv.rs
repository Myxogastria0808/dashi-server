use std::sync::Arc;

use axum::Extension;
use domain::entity::endpoint::EndPoint;

pub async fn depreiation_handler() -> String {
    "depreiation_handler".to_string()
}

pub async fn item_handler() -> String {
    "item_handler".to_string()
}
