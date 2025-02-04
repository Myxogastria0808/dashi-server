use crate::{generate::Generate, healthcheck::HealthCheck, item::register::RegisterItem};
use domain::{
    factory::shared_state::SharedStateFactory,
    repository::{
        generate::GenerateRepository, healthcheck::HealthCheckRepository,
        item::register::RegisterItemRepository,
    },
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct SharedState {
    pub register_item: RegisterItem,
    pub generate: Generate,
    pub healthcheck: HealthCheck,
}

impl SharedStateFactory for SharedState {
    async fn new() -> Self {
        SharedState {
            register_item: RegisterItem::new().await,
            generate: Generate::new().await,
            healthcheck: HealthCheck::new().await,
        }
    }
}
