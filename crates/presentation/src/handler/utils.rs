use application::utils::healthcheck::HealthCheckUseCase;
use axum::{debug_handler, extract::State};
use domain::{
    repository::healthcheck::HealthCheckRepository,
    value_object::{error::AppError, shared_state::RwLockSharedState},
};
use infrastructure::healthcheck::HealthCheck;

struct UseCase {
    healthcheck_usecase: HealthCheckUseCase<HealthCheck>,
}

impl UseCase {
    pub async fn new() -> Self {
        let healthcheck_usecase = HealthCheckUseCase::new(HealthCheck::new().await).await;
        Self {
            healthcheck_usecase,
        }
    }
    pub async fn healthcheck_usecase(&self) -> &HealthCheckUseCase<HealthCheck> {
        &self.healthcheck_usecase
    }
}

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
    //operation
    UseCase::new()
        .await
        .healthcheck_usecase()
        .await
        .healthcheck()
        .await?;
    drop(shared_model);
    Ok(())
}
