use crate::use_case_wrapper::generate::GenerateUseCaseWrapper;
use axum::{
    extract::{Path, State},
    Json,
};
use domain::value_object::{error::AppError, shared_state::RwLockSharedState};
use entity::label::Record;

pub async fn qr_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<Json<Vec<String>>, AppError> {
    tracing::info!("reached generate/qr handler handler.");
    tracing::info!("path (quantity): {}", quantity);
    //validation
    let shared_model = shared_state.write().await;
    //operation
    let use_case = GenerateUseCaseWrapper::new(quantity, Record::Qr).await;
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
    let use_case = GenerateUseCaseWrapper::new(quantity, Record::Barcode).await;
    //healthcheck
    use_case.healthcheck_usecase().await.healthcheck().await?;
    //generate
    let result = use_case.generate_usecase().await.generate().await?;
    drop(shared_model);
    Ok(Json(result))
}

pub async fn nothing_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<Json<Vec<String>>, AppError> {
    tracing::info!("reached generate/nothing handler handler.");
    tracing::info!("path (quantity): {}", quantity);
    //validation
    let shared_model = shared_state.write().await;
    //operation
    let use_case = GenerateUseCaseWrapper::new(quantity, Record::Nothing).await;
    //healthcheck
    use_case.healthcheck_usecase().await.healthcheck().await?;
    //generate
    let result = use_case.generate_usecase().await.generate().await?;
    drop(shared_model);
    Ok(Json(result))
}
