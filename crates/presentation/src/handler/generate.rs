use axum::extract::{Path, State};
use domain::value_object::shared_state::RwLockSharedState;

pub async fn qr_handler(
    Path(quantity): Path<u16>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    let shared_model = shared_state.write().await;
    drop(shared_model);
    "qr_handler".to_string()
}

pub async fn barcode_handler(
    Path(quantity): Path<u16>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    let shared_model = shared_state.write().await;
    drop(shared_model);
    "barcode_handler".to_string()
}
