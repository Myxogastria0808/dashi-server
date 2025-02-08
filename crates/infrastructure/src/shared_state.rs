use crate::{
    csv::depreiation::DepreiationCsv,
    generate::Generate,
    healthcheck::HealthCheck,
    item::{
        delete::DeleteItem, individual_item::IndividualItem, register::RegisterItem,
        search::SearchItem, update::UpdateItem,
    },
};
use domain::{
    factory::shared_state::SharedStateFactory,
    repository::{
        csv::depreiation::DepreiationCsvRepository,
        generate::GenerateRepository,
        healthcheck::HealthCheckRepository,
        item::{
            delete::DeleteItemRepository, individual::IndividualItemRepository,
            register::RegisterItemRepository, search::SearchItemRepository,
            update::UpdateItemRepository,
        },
    },
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct SharedState {
    pub depreiation_csv: DepreiationCsv,
    pub individual_item: IndividualItem,
    pub search_item: SearchItem,
    pub update_item: UpdateItem,
    pub delete_item: DeleteItem,
    pub register_item: RegisterItem,
    pub generate: Generate,
    pub healthcheck: HealthCheck,
}

impl SharedStateFactory for SharedState {
    async fn new() -> Self {
        SharedState {
            depreiation_csv: DepreiationCsv::new().await,
            individual_item: IndividualItem::new().await,
            search_item: SearchItem::new().await,
            update_item: UpdateItem::new().await,
            delete_item: DeleteItem::new().await,
            register_item: RegisterItem::new().await,
            generate: Generate::new().await,
            healthcheck: HealthCheck::new().await,
        }
    }
}
