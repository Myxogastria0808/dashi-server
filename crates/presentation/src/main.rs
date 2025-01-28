use async_std::sync::{Arc, RwLock};
use axum::{extract::DefaultBodyLimit, http::Method, routing::get, Router};
use domain::value_object::{
    error::api::ApiError,
    shared_state::{RwLockSharedState, SharedState},
};
use tower_http::cors::{Any, CorsLayer};
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

mod handler;
mod route;
mod use_case_wrapper;

#[tokio::main]
async fn main() {
    let _ = api().await;
}

//axum
async fn api() -> Result<(), ApiError> {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // Shared Object
    let shared_state: RwLockSharedState = Arc::new(RwLock::new(SharedState {}));
    // CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);
    // Router
    let app: Router<()> = Router::new()
        .route("/", get(ping))
        .merge(route::root::root_route())
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)) //100MB
        .with_state(Arc::clone(&shared_state));
    // Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    tracing::debug!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

//* dummy *//
async fn ping() -> String {
    tracing::info!("reached dummy handler.");
    "pong!".to_string()
}
