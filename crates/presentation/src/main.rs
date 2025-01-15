use axum::{Extension, Router, extract::DefaultBodyLimit, http::Method, routing::get};
use dotenvy::dotenv;
use std::env;
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
    // load .env file
    dotenv().expect(".env file not found.");
    // insert a environment variable
    let port: String = env::var("SERVER_PORT").expect("SERVER_PORT not found in .env file.");
    //CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);
    //Router
    let app = Router::new()
        .route("/", get(ping))
        .merge(route::root::root_route().await)
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)); //10MB
    //Server
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

//* dummy *//
async fn ping() -> String {
    "pong!".to_string()
}
