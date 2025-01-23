use axum::Extension;
use domain::entity::endpoint::EndPoint;

pub async fn qr_handler() -> String {
    "qr_handler".to_string()
}

pub async fn barcode_handler() -> String {
    "barcode_handler".to_string()
}
