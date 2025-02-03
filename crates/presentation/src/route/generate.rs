use crate::handler::generate::{barcode_handler, nothing_handler, qr_handler};
use application::shared_state::RwLockSharedState;
use axum::{routing::post, Router};

pub fn generate_route() -> Router<RwLockSharedState> {
    let generate_routes = Router::new()
        .route("/qr/:quantity", post(qr_handler))
        .route("/barcode/:quantity", post(barcode_handler))
        .route("/nothing/:quantity", post(nothing_handler));
    Router::new().nest("/generate", generate_routes)
}
