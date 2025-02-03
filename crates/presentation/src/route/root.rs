use crate::route::csv::csv_route;
use crate::route::generate::generate_route;
use crate::route::item::item_route;
use crate::route::rental::rent_route;
use crate::route::utils::util_route;
use application::shared_state::RwLockSharedState;
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
