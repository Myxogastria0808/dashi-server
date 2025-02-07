use application::usecase::utils::healthcheck::HealthCheckUseCase;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use domain::value_object::error::AppError;

use crate::models::rwlock_shared_state::RwLockSharedState;

pub async fn login_handler(State(shared_state): State<RwLockSharedState>) -> String {
    tracing::info!("reached utils/login handler.");
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "login_handler".to_string()
}

#[utoipa::path(
    get,
    path = "/api/utils/healthcheck",
    tag = "Health Check",
    responses(
        (status = 200, description = "OK"),
        (status = 500, description = "Internal Server Error")
    ),
)]
pub async fn healthcheck_handler(
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached utils/healthcheck handler.");
    let shared_model = shared_state.read().await;
    // operation
    let healthcheck_usecase = HealthCheckUseCase::new(shared_model.clone().healthcheck).await;
    healthcheck_usecase.run().await?;
    drop(shared_model);
    Ok((StatusCode::OK).into_response())
}
