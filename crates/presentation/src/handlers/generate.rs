use application::usecase::generate::{GenerateInputs, GenerateOutputs};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::value_object::error::AppError;
use entity::label::Record;

use crate::models::rwlock_shared_state::RwLockSharedState;

#[utoipa::path(
    post,
    path = "/api/generate/qr/{quantity}",
    params(("quantity", Path, description = "generate visible id's quantity")),
    tag = "Generate",
    responses(
        (status = 201, description = "CREATED", body = GenerateData),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn qr_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached generate/qr handler.");
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
    Ok((StatusCode::CREATED, Json(result)).into_response())
}

#[utoipa::path(
    post,
    path = "/api/generate/barcode/{quantity}",
    params(("quantity", Path, description = "generate visible id's quantity")),
    tag = "Generate",
    responses(
        (status = 201, description = "CREATED", body = GenerateData),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn barcode_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached generate/barcode handler.");
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
    Ok((StatusCode::CREATED, Json(result)).into_response())
}

#[utoipa::path(
    post,
    path = "/api/generate/nothing/{quantity}",
    params(("quantity", Path, description = "generate visible id's quantity")),
    tag = "Generate",
    responses(
        (status = 201, description = "CREATED", body = GenerateData),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn nothing_handler(
    Path(quantity): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached generate/nothing handler.");
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
    Ok((StatusCode::CREATED, Json(result)).into_response())
}
