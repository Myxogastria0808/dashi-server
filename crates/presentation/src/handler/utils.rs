use axum::extract::State;
use domain::value_object::shared_state::RwLockSharedState;

pub async fn login_handler(State(shared_state): State<RwLockSharedState>) -> String {
    //validation
    let shared_model = shared_state.read().unwrap();
    //operation
    drop(shared_model);
    "login_handler".to_string()
}

pub async fn healthcheck_handler(State(shared_state): State<RwLockSharedState>) -> String {
    //validation
    let shared_model = shared_state.read().unwrap();
    //operation
    drop(shared_model);
    "healthcheck_handler".to_string()
}
