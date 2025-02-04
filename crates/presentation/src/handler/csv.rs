use crate::RwLockSharedState;
use axum::extract::State;

pub async fn depreiation_handler(State(shared_state): State<RwLockSharedState>) -> String {
    tracing::info!("reached csv/depreiation handler.");
    let shared_model = shared_state.read().await;
    drop(shared_model);
    "depreiation_handler".to_string()
}

pub async fn item_handler(State(shared_state): State<RwLockSharedState>) -> String {
    tracing::info!("reached csv/item handler handler.");
    let shared_model = shared_state.read().await;
    drop(shared_model);
    "item_handler".to_string()
}
