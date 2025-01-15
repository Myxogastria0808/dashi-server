use crate::handler::rent::{rent_handler, return_handler};
use axum::{
    Router,
    routing::{post, put},
};

pub async fn rent_route() -> Router {
    let rent_routes = Router::new()
        .route("/rent", post(rent_handler))
        .route("/return/:visible_id", put(return_handler));
    Router::new().merge(Router::new().nest("/rent", rent_routes))
}
