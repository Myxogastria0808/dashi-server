use axum::{
    extract::{Path, State},
    Json,
};
use domain::{
    entity::data_type::rent_item::RentItemData, value_object::shared_state::RwLockSharedState,
};

pub async fn rent_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(register_data): Json<RentItemData>,
) -> String {
    tracing::info!("reached rental/rent handler.");
    tracing::info!("body (register_data): {:?}", register_data);
    let shared_model = shared_state.write().await;
    drop(shared_model);
    "rent_handler".to_string()
}

pub async fn render_handler(
    Path(visible_id): Path<String>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached rental/render handler.");
    tracing::info!("path (visible_id): {}", visible_id);
    let shared_model = shared_state.read().await;
    drop(shared_model);
    "return_handler".to_string()
}
