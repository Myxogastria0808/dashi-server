use domain::{
    repository::{generate::GenerateRepository, healthcheck::HealthCheckRepository},
    value_object::error::{healthcheck::HealthCheckError, AppError},
};

pub struct GenerateUseCase<T: HealthCheckRepository, S: GenerateRepository> {
    healyhcheck_repository: T,
    generate_repository: S,
}

impl<T: HealthCheckRepository, S: GenerateRepository> GenerateUseCase<T, S> {
    pub async fn new(healyhcheck_repository: T, generate_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            generate_repository,
        }
    }
    pub async fn healthcheck(&self) -> Result<(), HealthCheckError> {
        self.healyhcheck_repository.healthcheck().await
    }
    pub async fn generate(&self) -> Result<Vec<String>, AppError> {
        self.generate_repository.generate().await
    }
}
