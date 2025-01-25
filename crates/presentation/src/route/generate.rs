use crate::handler::generate::{barcode_handler, qr_handler};
use axum::{Router, routing::post};
use domain::value_object::shared_state::RwLockSharedState;

pub fn generate_route() -> Router<RwLockSharedState> {
    let generate_routes = Router::new()
        .route("/qr", post(qr_handler))
        .route("/barcode", post(barcode_handler));
    Router::new().nest("/generate", generate_routes)
}
