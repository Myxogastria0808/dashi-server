use application::item::register::RegisterItemUseCase;
use domain::{
    entity::data_type::register_item::RegisterItemData,
    repository::{healthcheck::HealthCheckRepository, item::register::RegisterItemRepository},
};
use infrastructure::{healthcheck::HealthCheck, item::RegisterItem};

pub struct RegisterItemUseCaseWrapper {
    pub register_usecase: RegisterItemUseCase<HealthCheck, RegisterItem>,
}

impl RegisterItemUseCaseWrapper {
    pub async fn new(register_item_data: RegisterItemData) -> Self {
        let register_usecase = RegisterItemUseCase::<HealthCheck, RegisterItem>::new(
            HealthCheck::new().await,
            RegisterItem::new(register_item_data).await,
        )
        .await;
        Self { register_usecase }
    }
    pub async fn healthcheck_usecase(&self) -> &RegisterItemUseCase<HealthCheck, RegisterItem> {
        &self.register_usecase
    }
    pub async fn register_usecase(&self) -> &RegisterItemUseCase<HealthCheck, RegisterItem> {
        &self.register_usecase
    }
}
