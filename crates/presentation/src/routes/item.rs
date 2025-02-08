use crate::{
    handlers::item::{
        archive_handler, cable_handler, connctor_handler, delete_handler, individual_item_handler,
        register_handler, search_handler, transfer_handler, update_handler,
    },
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn item_route() -> Router<RwLockSharedState> {
    let item_routes = Router::new()
        .route("/search", get(search_handler))
        .route("/connector/:connector_name/search", get(connctor_handler))
        .route("/cable/:cable_color_pattern/search", get(cable_handler))
        .route("/:id", get(individual_item_handler))
        .route("/archive/:limit", get(archive_handler))
        .route("/register", post(register_handler))
        .route("/update/:id", put(update_handler))
        .route("/delete/:id", delete(delete_handler))
        .route("/transfer", put(transfer_handler));
    Router::new().nest("/item", item_routes)
}
