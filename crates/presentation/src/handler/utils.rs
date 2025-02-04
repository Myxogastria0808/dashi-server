use crate::RwLockSharedState;
use application::usecase::utils::healthcheck::HealthCheckUseCase;
use axum::{debug_handler, extract::State};
use domain::value_object::error::AppError;

pub async fn login_handler(State(shared_state): State<RwLockSharedState>) -> String {
    tracing::info!("reached utils/login handler.");
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "login_handler".to_string()
}

#[debug_handler]
pub async fn healthcheck_handler(
    State(shared_state): State<RwLockSharedState>,
) -> Result<(), AppError> {
    tracing::info!("reached utils/healthcheck handler.");
    let shared_model = shared_state.read().await;
    // operation
    let healthcheck_usecase = HealthCheckUseCase::new(shared_model.clone().healthcheck).await;
    healthcheck_usecase.run().await?;
    drop(shared_model);
    Ok(())
}
