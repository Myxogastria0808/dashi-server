use std::sync::{Arc, RwLock};

use axum::{Extension, extract::State};
use domain::entity::endpoint::EndPoint;

use crate::SharedState;

pub async fn login_handler() -> String {
    "login_handler".to_string()
}

pub async fn healthcheck_handler(State(shared_state): State<Arc<RwLock<SharedState>>>) -> String {
    shared_state.write().unwrap();
    "aa".to_string()
}
