use crate::handler::csv::{depreiation_handler, item_handler};
use axum::{Router, routing::get};
use domain::value_object::shared_state::RwLockSharedState;

pub fn csv_route() -> Router<RwLockSharedState> {
    let csv_routes = Router::new()
        .route("/", get(depreiation_handler))
        .route("/item", get(item_handler));
    Router::new().nest("/csv", csv_routes)
}
