use application::{
    shared_state::RwLockSharedState,
    utils::healthcheck::{HealthCheckInputs, HealthCheckOutputs},
};
use axum::{debug_handler, extract::State};
use domain::{repository::healthcheck::HealthCheckRepository, value_object::error::AppError};
use infrastructure::healthcheck::HealthCheck;

pub async fn login_handler(State(shared_state): State<RwLockSharedState>) -> String {
    tracing::info!("reached utils/login handler.");
    //validation
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
    //validation
    let shared_model = shared_state.read().await;
    let healthcheck_inputs = HealthCheckInputs;
    let healthcheck_outputs = HealthCheckOutputs::new(HealthCheck::new().await).await;
    healthcheck_outputs.run(healthcheck_inputs).await?;
    drop(shared_model);
    Ok(())
}
