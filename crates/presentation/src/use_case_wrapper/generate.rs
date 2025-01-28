use application::generate::GenerateUseCase;
use domain::repository::{generate::GenerateRepository, healthcheck::HealthCheckRepository};
use entity::label::Record;
use infrastructure::{generate::Generate, healthcheck::HealthCheck};

pub(crate) struct GenerateUseCaseWrapper {
    generate_usecase: GenerateUseCase<HealthCheck, Generate>,
}

impl GenerateUseCaseWrapper {
    pub async fn new(quantity: u32, qr_or_barcode: Record) -> Self {
        let generate_usecase = GenerateUseCase::<HealthCheck, Generate>::new(
            HealthCheck::new().await,
            Generate::new(quantity, qr_or_barcode).await,
        )
        .await;
        Self { generate_usecase }
    }
    pub async fn healthcheck_usecase(&self) -> &GenerateUseCase<HealthCheck, Generate> {
        &self.generate_usecase
    }
    pub async fn generate_usecase(&self) -> &GenerateUseCase<HealthCheck, Generate> {
        &self.generate_usecase
    }
}
