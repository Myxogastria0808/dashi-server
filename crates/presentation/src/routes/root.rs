use crate::models::rwlock_shared_state::RwLockSharedState;
use crate::routes::csv::csv_route;
use crate::routes::generate::generate_route;
use crate::routes::item::item_route;
use crate::routes::rental::rent_route;
use crate::routes::utils::util_route;
use axum::Router;

pub fn root_route() -> Router<RwLockSharedState> {
    let root_routes = Router::new()
        .merge(csv_route())
        .merge(generate_route())
        .merge(item_route())
        .merge(rent_route())
        .merge(util_route());
    Router::new().nest("/api", root_routes)
}
