use crate::route::csv::csv_route;
use crate::route::generate::generate_route;
use crate::route::item::item_route;
use crate::route::rent::rent_route;
use crate::route::utils::util_route;
use axum::Router;

pub async fn root_route() -> Router {
    let root_routes = Router::new()
        .merge(csv_route().await)
        .merge(generate_route().await)
        .merge(item_route().await)
        .merge(rent_route().await)
        .merge(util_route().await);
    Router::new().merge(Router::new().nest("/api", root_routes))
}
