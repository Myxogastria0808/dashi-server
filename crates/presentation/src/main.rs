use axum::{Router, extract::DefaultBodyLimit, http::Method, routing::get};
use domain::{
    repository::connection::ConnectionRepository,
    value_object::{
        error::{AppError, connection::ConnectionError},
        shared_state::{RwLockSharedState, SharedState},
    },
};
use infrastructure::connection::CollectConnection;
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

//SharedStateModel is a collection of infrastructure connections to abstraction of infrastructure objects.
//The presentation and infrastructure layers rely on this Model to achieve DI.
struct SharedStateModel {
    shared_state: SharedState,
}

impl SharedStateModel {
    pub async fn new() -> Result<Self, ConnectionError> {
        let connection = CollectConnection::new().await?;
        let shared_state = SharedState {
            graph_db: connection.connect_neo4j().await?,
            meilisearch: connection.connect_meilisearch().await?,
            rdb: connection.connect_postgres().await?,
        };
        Ok(SharedStateModel { shared_state })
    }
}

//axum
async fn api() -> Result<(), AppError> {
    //Shared Object
    let shared_state_model = SharedStateModel::new().await?;
    let shared_state: RwLockSharedState = Arc::new(RwLock::new(shared_state_model.shared_state));
    //CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::DELETE, Method::PUT])
        .allow_origin(Any);
    //Router
    let app: Router<()> = Router::new()
        .route("/", get(ping))
        .merge(route::root::root_route())
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)) //100MB
        .with_state(Arc::clone(&shared_state));
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
