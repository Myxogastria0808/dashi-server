use crate::handler::generate::{barcode_handler, qr_handler};
use axum::{Router, routing::post};

pub async fn generate_route() -> Router {
    let generate_routes = Router::new()
        .route("/qr", post(qr_handler))
        .route("/barcode", post(barcode_handler));
    Router::new().merge(Router::new().nest("/generate", generate_routes))
}
