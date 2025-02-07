use application::model::shared_state::SharedStateUseCase;
use async_std::sync::{Arc, RwLock};
use axum::{extract::DefaultBodyLimit, http::Method, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// レイヤードアーキテクチャに違反しているが、Rustの性質上不可能なのでinfrastructure層及びdomain層から直接呼び出す
use crate::{error::api::ApiError, routes};
use domain::factory::shared_state::SharedStateFactory;
use infrastructure::shared_state::SharedState;

//axum
pub async fn api() -> Result<(), ApiError> {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // Shared Object
    let shared_state = Arc::new(RwLock::new(
        SharedStateUseCase::new(SharedState::new().await)
            .await
            .shared_state_factory,
    ));
    // CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_origin(Any);
    // Router
    let app: Router<()> = Router::new()
        .route("/", get(ping))
        .merge(routes::root::root_route())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
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

#[derive(OpenApi)]
#[openapi(
    info(
        title = "dashi-server",
        version = "0.0.1",
        description = "Thsi is a dashi-server API document.",
        contact(
            name = "Myxogastria0808",
            email = "r.rstudio.c@gmail.com",
            url = "https://yukiosada.work",
        ),
        license(
            name = "WTFPL",
            url = "http://www.wtfpl.net"
        ),
    ),
    servers((url = "http://localhost:5000")),
    tags(
        (name = "Health Check", description = "Health Checkのエンドポイント"),
        // (name = "Item", description = "物品に関係するエンドポイント"),
        (name = "Generate", description = "QRまたはBarcodeを生成するエンドポイント"),
    ),
    paths(
        crate::handlers::generate::qr_handler,
        crate::handlers::generate::barcode_handler,
        crate::handlers::generate::nothing_handler,
        crate::handlers::utils::healthcheck_handler,
    ),
    components(schemas(
        domain::entity::data_type::generate::GenerateData,
    ))
)]
struct ApiDoc;
