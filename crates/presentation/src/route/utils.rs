use crate::handler::utils::{healthcheck_handler, login_handler};
use axum::{
    Router,
    routing::{get, post},
};

pub async fn util_route() -> Router {
    let utils_routes = Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .route("/login", post(login_handler));
    Router::new().merge(Router::new().nest("/utils", utils_routes))
}
