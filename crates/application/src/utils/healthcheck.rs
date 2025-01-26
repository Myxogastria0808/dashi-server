use domain::{
    repository::healthcheck::HealthCheckRepository,
    value_object::error::healthcheck::HealthCheckError,
};

pub struct HealthCheckUseCase<T: HealthCheckRepository> {
    healyhcheck_repository: T,
}

impl<T: HealthCheckRepository> HealthCheckUseCase<T> {
    pub async fn new(healyhcheck_repository: T) -> Self {
        Self {
            healyhcheck_repository,
        }
    }
    pub async fn healthcheck(&self) -> Result<(), HealthCheckError> {
        self.healyhcheck_repository.healthcheck().await
    }
}
