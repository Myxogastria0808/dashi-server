use crate::handlers::utils::{generate_handler, healthcheck_handler, login_handler};
use crate::models::rwlock_shared_state::RwLockSharedState;
use crate::routes::csv::csv_route;
use crate::routes::item::item_route;
use crate::routes::joke::joke_route;
use crate::routes::rental::rent_route;
use axum::routing::{get, post};
use axum::Router;

pub fn util_route() -> Router<RwLockSharedState> {
    Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .route("/login", post(login_handler))
        .route("/generate", post(generate_handler))
}

pub fn root_route() -> Router<RwLockSharedState> {
    let root_routes = Router::new()
        .merge(csv_route())
        .merge(item_route())
        .merge(rent_route())
        .merge(util_route())
        .merge(joke_route());
    Router::new().nest("/api", root_routes)
}
