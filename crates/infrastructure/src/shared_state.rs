use crate::{
    generate::Generate,
    healthcheck::HealthCheck,
    item::{delete::DeleteItem, register::RegisterItem},
};
use domain::{
    factory::shared_state::SharedStateFactory,
    repository::{
        generate::GenerateRepository,
        healthcheck::HealthCheckRepository,
        item::{delete::DeleteItemRepository, register::RegisterItemRepository},
    },
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct SharedState {
    pub delete_item: DeleteItem,
    pub register_item: RegisterItem,
    pub generate: Generate,
    pub healthcheck: HealthCheck,
}

impl SharedStateFactory for SharedState {
    async fn new() -> Self {
        SharedState {
            delete_item: DeleteItem::new().await,
            register_item: RegisterItem::new().await,
            generate: Generate::new().await,
            healthcheck: HealthCheck::new().await,
        }
    }
}
