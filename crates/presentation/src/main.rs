use axum::{Extension, Router, extract::DefaultBodyLimit, http::Method, routing::get};
use domain::entity::endpoint::EndPoint;
use std::sync::{Arc, RwLock};
use tower_http::cors::{Any, CorsLayer};
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

mod handler;
mod route;

#[tokio::main]
async fn main() {
    let _ = api().await;
}

//axum
async fn api() -> Result<(), axum::Error> {
    //Shared Object
    let data = Arc::new(RwLock::new(EndPoint::default()));
    //CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);
    //Router
    let app = Router::new()
        .route("/", get(ping))
        .merge(route::root::root_route().await)
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(Extension(data))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)); //10MB
    //Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

//* dummy *//
async fn ping() -> String {
    "pong!".to_string()
}
