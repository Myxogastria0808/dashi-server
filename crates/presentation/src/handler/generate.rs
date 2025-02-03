use application::{
    generate::{GenerateInputs, GenerateOutputs},
    shared_state::RwLockSharedState,
};
use axum::{
    extract::{Path, State},
    Json,
};
use domain::{
    repository::{generate::GenerateRepository, healthcheck::HealthCheckRepository},
    value_object::error::AppError,
};
use entity::label::Record;
use infrastructure::{generate::Generate, healthcheck::HealthCheck};

pub async fn qr_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<Json<Vec<String>>, AppError> {
    tracing::info!("reached generate/qr handler handler.");
    tracing::info!("path (quantity): {}", quantity);
    let shared_model = shared_state.write().await;
    // operation
    let generate_inputs = GenerateInputs {
        quantity,
        record: Record::Qr,
    };
    let generate_outputs =
        GenerateOutputs::new(HealthCheck::new().await, Generate::new().await).await;
    let result = generate_outputs.run(generate_inputs).await?;
    drop(shared_model);
    Ok(Json(result))
}

pub async fn barcode_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<Json<Vec<String>>, AppError> {
    tracing::info!("reached generate/barcode handler handler.");
    tracing::info!("path (quantity): {}", quantity);
    let shared_model = shared_state.write().await;
    // operation
    let generate_inputs = GenerateInputs {
        quantity,
        record: Record::Barcode,
    };
    let generate_outputs =
        GenerateOutputs::new(HealthCheck::new().await, Generate::new().await).await;
    let result = generate_outputs.run(generate_inputs).await?;
    drop(shared_model);
    Ok(Json(result))
}

pub async fn nothing_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<Json<Vec<String>>, AppError> {
    tracing::info!("reached generate/nothing handler handler.");
    tracing::info!("path (quantity): {}", quantity);
    let shared_model = shared_state.write().await;
    // operation
    let generate_inputs = GenerateInputs {
        quantity,
        record: Record::Nothing,
    };
    let generate_outputs =
        GenerateOutputs::new(HealthCheck::new().await, Generate::new().await).await;
    let result = generate_outputs.run(generate_inputs).await?;
    drop(shared_model);
    Ok(Json(result))
}
