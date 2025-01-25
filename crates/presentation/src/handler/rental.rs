use axum::{
    Json,
    extract::{Path, State},
};
use domain::{
    entity::data_type::rent_item::RentItemData, value_object::shared_state::RwLockSharedState,
};

pub async fn rent_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(register_data): Json<RentItemData>,
) -> String {
    let shared_model = shared_state.write().unwrap();
    drop(shared_model);
    "rent_handler".to_string()
}

pub async fn return_handler(
    Path(visible_id): Path<String>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    let shared_model = shared_state.read().unwrap();
    drop(shared_model);
    "return_handler".to_string()
}
