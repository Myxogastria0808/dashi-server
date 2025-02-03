use domain::{
    repository::healthcheck::HealthCheckRepository,
    value_object::error::healthcheck::HealthCheckError,
};

#[derive(Default)]
pub struct HealthCheckInputs;

pub struct HealthCheckOutputs<T: HealthCheckRepository> {
    healthcheck_interface: T,
}

impl<T: HealthCheckRepository> HealthCheckOutputs<T> {
    pub async fn new(healthcheck_interface: T) -> Self {
        Self {
            healthcheck_interface,
        }
    }
    pub async fn run(
        &self,
        _healthcheck_interface: HealthCheckInputs,
    ) -> Result<(), HealthCheckError> {
        self.healthcheck_interface.healthcheck().await
    }
}
