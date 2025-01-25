use crate::handler::rental::{rent_handler, return_handler};
use axum::{
    Router,
    routing::{post, put},
};
use domain::value_object::shared_state::RwLockSharedState;

pub fn rent_route() -> Router<RwLockSharedState> {
    let rent_routes = Router::new()
        .route("/rent", post(rent_handler))
        .route("/return/:visible_id", put(return_handler));
    Router::new().nest("/rent", rent_routes)
}
