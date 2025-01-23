use axum::Extension;
use domain::entity::endpoint::EndPoint;

pub async fn search_handler() -> String {
    "search_handler".to_string()
}

pub async fn each_item_handler() -> String {
    "each_item_handler".to_string()
}

pub async fn connctor_handler() -> String {
    "connctor_handler".to_string()
}

pub async fn cable_handler() -> String {
    "cable_handler".to_string()
}

pub async fn register_handler() -> String {
    "register_handler".to_string()
}

pub async fn update_handler() -> String {
    "update_handler".to_string()
}

pub async fn delete_handler() -> String {
    "delete_handler".to_string()
}
