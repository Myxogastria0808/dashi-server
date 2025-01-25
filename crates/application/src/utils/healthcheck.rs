use domain::{
    repository::healthcheck::HealthCheckRepository,
    value_object::error::healthcheck::HealthCheckError,
};

pub struct HealthCheckUseCase<T: HealthCheckRepository> {
    healyhcheck_repository: T,
}

impl<T: HealthCheckRepository> HealthCheckUseCase<T> {
    pub async fn hrealthcheck(&self) -> Result<(), HealthCheckError> {
        self.healthcheck_graphdb().await?;
        self.healthcheck_meilisearch().await?;
        self.healthcheck_rdb().await?;
        Ok(())
    }
    async fn healthcheck_graphdb(&self) -> Result<(), HealthCheckError> {
        self.healyhcheck_repository.healthcheck_graphdb().await
    }
    async fn healthcheck_meilisearch(&self) -> Result<(), HealthCheckError> {
        self.healyhcheck_repository.healthcheck_meilisearch().await
    }
    async fn healthcheck_rdb(&self) -> Result<(), HealthCheckError> {
        self.healyhcheck_repository.healthcheck_rdb().await
    }
}
