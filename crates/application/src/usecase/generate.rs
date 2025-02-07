use domain::{
    entity::data_type::generate::GenerateData,
    repository::{
        generate::{GenerateInterface, GenerateRepository},
        healthcheck::HealthCheckRepository,
    },
    value_object::error::AppError,
};
use entity::label::Record;

pub struct GenerateInputs {
    pub quantity: u32,
    pub record: Record,
}

pub struct GenerateOutputs<T: HealthCheckRepository, S: GenerateRepository> {
    healthcheck_repository: T,
    generate_repository: S,
}

impl<T: HealthCheckRepository, S: GenerateRepository> GenerateOutputs<T, S> {
    pub async fn new(healthcheck_repository: T, generate_repository: S) -> Self {
        Self {
            healthcheck_repository,
            generate_repository,
        }
    }
    pub async fn run(&self, generate_inputs: GenerateInputs) -> Result<GenerateData, AppError> {
        self.healthcheck_repository.healthcheck().await?;
        let generate_interface =
            GenerateInterface::new(generate_inputs.quantity, generate_inputs.record).await;
        self.generate_repository.generate(generate_interface).await
    }
}
