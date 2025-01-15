use crate::handler::csv::{depreiation_handler, item_handler};
use axum::{Router, routing::get};

pub async fn csv_route() -> Router {
    let csv_routes = Router::new()
        .route("/", get(depreiation_handler))
        .route("/item", get(item_handler));
    Router::new().merge(Router::new().nest("/csv", csv_routes))
}
