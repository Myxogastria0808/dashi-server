use domain::{
    repository::{healthcheck::HealthCheckRepository, item::register::RegisterItemRepository},
    value_object::error::{healthcheck::HealthCheckError, AppError},
};

pub struct RegisterItemUseCase<T: HealthCheckRepository, S: RegisterItemRepository> {
    healyhcheck_repository: T,
    register_item_repository: S,
}

impl<T: HealthCheckRepository, S: RegisterItemRepository> RegisterItemUseCase<T, S> {
    pub async fn new(healyhcheck_repository: T, register_item_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            register_item_repository,
        }
    }
    pub async fn healthcheck(&self) -> Result<(), HealthCheckError> {
        self.healyhcheck_repository.healthcheck().await
    }
    pub async fn register(&self) -> Result<(), AppError> {
        self.register_item_repository.register().await
    }
}
