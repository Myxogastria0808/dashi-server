use crate::handler::utils::{healthcheck_handler, login_handler};
use axum::{
    Router,
    routing::{get, post},
};
use domain::value_object::shared_state::RwLockSharedState;

pub fn util_route() -> Router<RwLockSharedState> {
    let utils_routes = Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .route("/login", post(login_handler));
    Router::new().nest("/utils", utils_routes)
}
