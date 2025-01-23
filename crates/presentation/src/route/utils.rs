use crate::handler::utils::{healthcheck_handler, login_handler};
use axum::{
    Router,
    routing::{get, post},
};

pub fn util_route<S>() -> Router<S>
where
    S: Send + Sync + Clone + 'static,
{
    let utils_routes = Router::new().route("/healthcheck", get(healthcheck_handler));
    // .route("/login", post(login_handler));
    Router::new().nest("/utils", utils_routes)
}
