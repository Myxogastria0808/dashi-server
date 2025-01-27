use application::generate::GenerateUseCase;
use axum::{
    extract::{Path, State},
    Json,
};
use domain::{
    repository::{generate::GenerateRepository, healthcheck::HealthCheckRepository},
    value_object::{error::AppError, shared_state::RwLockSharedState},
};
use entity::label::Record;
use infrastructure::{generate::Generate, healthcheck::HealthCheck};

struct UseCase {
    generate_usecase: GenerateUseCase<HealthCheck, Generate>,
}

impl UseCase {
    pub async fn new(quantity: u32, qr_or_barcode: Record) -> Self {
        let generate_usecase = GenerateUseCase::<HealthCheck, Generate>::new(
            HealthCheck::new().await,
            Generate::new(quantity, qr_or_barcode).await,
        )
        .await;
        Self { generate_usecase }
    }
    pub async fn healthcheck_usecase(&self) -> &GenerateUseCase<HealthCheck, Generate> {
        &self.generate_usecase
    }
    pub async fn generate_usecase(&self) -> &GenerateUseCase<HealthCheck, Generate> {
        &self.generate_usecase
    }
}

pub async fn qr_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<Json<Vec<String>>, AppError> {
    tracing::info!("reached generate/qr handler handler.");
    tracing::info!("path (quantity): {}", quantity);
    //validation
    let shared_model = shared_state.write().await;
    //operation
    let use_case = UseCase::new(quantity, Record::Qr).await;
    //healthcheck
    use_case.healthcheck_usecase().await.healthcheck().await?;
    //generate
    let result = use_case.generate_usecase().await.generate().await?;
    drop(shared_model);
    Ok(Json(result))
}

pub async fn barcode_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<Json<Vec<String>>, AppError> {
    tracing::info!("reached generate/barcode handler handler.");
    tracing::info!("path (quantity): {}", quantity);
    //validation
    let shared_model = shared_state.write().await;
    //operation
    let use_case = UseCase::new(quantity, Record::Barcode).await;
    //healthcheck
    use_case.healthcheck_usecase().await.healthcheck().await?;
    //generate
    let result = use_case.generate_usecase().await.generate().await?;
    drop(shared_model);
    Ok(Json(result))
}
