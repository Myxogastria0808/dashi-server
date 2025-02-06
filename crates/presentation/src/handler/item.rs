use crate::RwLockSharedState;
use application::usecase::item::{
    delete::{DeleteItemInputs, DeleteItemOutputs},
    register::{RegisterItemInputs, RegisterItemOutputs},
    update::{UpdateItemDataJson, UpdateItemInputs, UpdateItemOutputs},
};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    Json,
};
use domain::{
    entity::data_type::{register_item::RegisterItemData, transfer_item::TransferItemData},
    value_object::error::AppError,
};
use std::collections::HashMap;

pub async fn search_handler(
    Query(param): Query<HashMap<String, String>>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/search handler.");
    tracing::info!("query (keywords): {:?}", param.get("keywords"));
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
    tracing::info!("reached item/each_item handler.");
    tracing::info!("path (visible_id): {}", visible_id);
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "each_item_handler".to_string()
}

pub async fn connctor_handler(
    Query(param): Query<HashMap<String, String>>,
    Path(connector_name): Path<String>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/connector handler.");
    tracing::info!("path (connector_name): {}", connector_name);
    tracing::info!("query (keywords): {:?}", param.get("keywords"));
    let keywords = match param.get("keywords") {
        Some(keywords) => keywords,
        None => "",
    };
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "connctor_handler".to_string()
}

pub async fn cable_handler(
    Path(cable_color_pattern): Path<String>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/cable handler.");
    tracing::info!("path (cable_color_pattern): {}", cable_color_pattern);
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "cable_handler".to_string()
}

pub async fn delete_history_handler(
    Path(limit): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/delete-history handler.");
    tracing::info!("path (limit): {}", limit);
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "delete_history_handler".to_string()
}

pub async fn register_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(register_item_data): Json<RegisterItemData>,
) -> Result<(), AppError> {
    tracing::info!("reached item/register handler.");
    tracing::info!("body (register_data): {:?}", register_item_data);
    let shared_model = shared_state.write().await;
    // operation
    let inputs = RegisterItemInputs { register_item_data };
    let outputs = RegisterItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().register_item,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok(())
}

pub async fn update_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
    Json(update_item_data_json): Json<UpdateItemDataJson>,
) -> Result<(), AppError> {
    tracing::info!("reached item/update handler.");
    tracing::info!("path (id): {}", id);
    tracing::info!("body (update_item_data_json): {:?}", update_item_data_json);
    let shared_model = shared_state.write().await;
    //operation
    let inputs = UpdateItemInputs {
        id,
        update_item_data_json,
    };
    let outputs = UpdateItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().update_item,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok(())
}

pub async fn delete_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<(), AppError> {
    tracing::info!("reached item/delete handler.");
    tracing::info!("path (id): {}", id);
    let shared_model = shared_state.write().await;
    //operation
    let inputs = DeleteItemInputs { id };
    let outputs = DeleteItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().delete_item,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok(())
}

#[axum::debug_handler]
pub async fn transfer_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(transfer_item_data): Json<Vec<TransferItemData>>,
) -> Result<(), AppError> {
    tracing::info!("reached item/transfer handler.");
    tracing::info!("body (transfer_item_data_inputs): {:?}", transfer_item_data);
    let shared_model = shared_state.write().await;
    //operation
    // let inputs = TransferItemInputs { transfer_item_data };
    // let outputs = TransferItemOutputs::new(
    //     shared_model.clone().healthcheck,
    //     shared_model.clone().move_item,
    // )
    // .await;
    // outputs.run(inputs).await?;
    drop(shared_model);
    Ok(())
}
