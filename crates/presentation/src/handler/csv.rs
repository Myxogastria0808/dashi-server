use axum::extract::State;
use domain::value_object::shared_state::RwLockSharedState;

pub async fn depreiation_handler(State(shared_state): State<RwLockSharedState>) -> String {
    let shared_model = shared_state.read().await;
    drop(shared_model);
    "depreiation_handler".to_string()
}

pub async fn item_handler(State(shared_state): State<RwLockSharedState>) -> String {
    let shared_model = shared_state.read().await;
    drop(shared_model);
    "item_handler".to_string()
}
