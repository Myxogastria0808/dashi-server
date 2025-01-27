use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use domain::{
    entity::data_type::{register_item::RegisterItemData, update_item::UpdateItemData},
    value_object::shared_state::RwLockSharedState,
};

pub async fn search_handler(
    Query(param): Query<HashMap<String, String>>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/search handler handler.");
    tracing::info!("query (keywords): {:?}", param.get("keywords"));
    //validation
    let keywords = match param.get("keywords") {
        Some(keywords) => keywords,
        None => "",
    };
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "search_handler".to_string()
}

pub async fn each_item_handler(
    Path(visible_id): Path<String>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/each_item handler handler.");
    tracing::info!("path (visible_id): {}", visible_id);
    //validation
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "each_item_handler".to_string()
}

pub async fn connctor_handler(
    Path(connector_name): Path<String>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/connector handler handler.");
    tracing::info!("path (connector_name): {}", connector_name);
    //validation
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "connctor_handler".to_string()
}

pub async fn cable_handler(
    Path(cable_color_pattern): Path<String>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/cable handler handler.");
    tracing::info!("path (cable_color_pattern): {}", cable_color_pattern);
    //validation
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "cable_handler".to_string()
}

pub async fn register_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(register_data): Json<RegisterItemData>,
) -> String {
    tracing::info!("reached item/register handler handler.");
    tracing::info!("body (register_data): {:?}", register_data);
    //validation
    let shared_model = shared_state.write().await;
    //operation
    drop(shared_model);
    "register_handler".to_string()
}

pub async fn update_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(update_data): Json<UpdateItemData>,
) -> String {
    tracing::info!("reached item/update handler handler.");
    tracing::info!("body (update_data): {:?}", update_data);
    //validation
    let shared_model = shared_state.write().await;
    //operation
    drop(shared_model);
    "update_handler".to_string()
}

pub async fn delete_handler(
    Path(visible_id): Path<String>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/delete handler handler.");
    tracing::info!("path (visible_id): {}", visible_id);
    //validation
    let shared_model = shared_state.write().await;
    //operation
    drop(shared_model);
    "delete_handler".to_string()
}
