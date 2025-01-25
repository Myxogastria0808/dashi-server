use crate::handler::rental::{render_handler, rent_handler};
use axum::{
    Router,
    routing::{post, put},
};
use domain::value_object::shared_state::RwLockSharedState;

pub fn rent_route() -> Router<RwLockSharedState> {
    let rent_routes = Router::new()
        .route("/rent", post(rent_handler))
        .route("/return/:visible_id", put(render_handler));
    Router::new().nest("/rent", rent_routes)
}
