use crate::handler::item::{
    cable_handler, connctor_handler, delete_handler, each_item_handler, register_handler,
    search_handler, update_handler,
};
use axum::{
    Router,
    routing::{get, post, put},
};
use domain::value_object::shared_state::RwLockSharedState;

pub fn item_route() -> Router<RwLockSharedState> {
    let item_routes = Router::new()
        .route("/search", get(search_handler))
        .route("/:visible_id", get(each_item_handler))
        .route("/connector/:connector_name", get(connctor_handler))
        .route("/cable", get(cable_handler))
        .route("/register", post(register_handler))
        .route("/update", put(update_handler))
        .route("/delete/:visible_id", post(delete_handler));
    Router::new().nest("/item", item_routes)
}
