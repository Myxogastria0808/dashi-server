use crate::handler::item::{
    cable_handler, connctor_handler, delete_handler, each_item_handler, register_handler,
    search_handler, update_handler,
};
use application::shared_state::RwLockSharedState;
use axum::{
    routing::{get, post, put},
    Router,
};

pub fn item_route() -> Router<RwLockSharedState> {
    let item_routes = Router::new()
        .route("/search", get(search_handler))
        .route("/:visible_id", get(each_item_handler))
        .route("/connector/:connector_name/search", get(connctor_handler))
        .route("/cable", get(cable_handler))
        .route("/register", post(register_handler))
        .route("/update", put(update_handler))
        .route("/delete/:visible_id", post(delete_handler));
    Router::new().nest("/item", item_routes)
}
