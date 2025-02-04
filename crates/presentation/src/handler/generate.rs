use crate::RwLockSharedState;
use application::usecase::generate::{GenerateInputs, GenerateOutputs};
use axum::{
    extract::{Path, State},
    Json,
};
use domain::value_object::error::AppError;
use entity::label::Record;

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
    let generate_outputs = GenerateOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().generate,
    )
    .await;
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
    let generate_outputs = GenerateOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().generate,
    )
    .await;
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
    let generate_outputs = GenerateOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().generate,
    )
    .await;
    let result = generate_outputs.run(generate_inputs).await?;
    drop(shared_model);
    Ok(Json(result))
}
