use crate::{
    handler::utils::{healthcheck_handler, login_handler},
    RwLockSharedState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn util_route() -> Router<RwLockSharedState> {
    let utils_routes = Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .route("/login", post(login_handler));
    Router::new().nest("/utils", utils_routes)
}
