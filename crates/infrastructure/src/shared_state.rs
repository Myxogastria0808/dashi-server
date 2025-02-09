use crate::{
    csv::{depreiation::DepreiationCsv, item::ItemCsv},
    generate::Generate,
    healthcheck::HealthCheck,
    item::{
        delete::DeleteItem, individual_item::IndividualItem, register::RegisterItem,
        search::SearchItem, transfer::TransferItem, update::UpdateItem,
    },
};
use domain::{
    factory::shared_state::SharedStateFactory,
    repository::{
        csv::{depreiation::DepreiationCsvRepository, item::ItemCsvRepository},
        generate::GenerateRepository,
        healthcheck::HealthCheckRepository,
        item::{
            delete::DeleteItemRepository, individual::IndividualItemRepository,
            register::RegisterItemRepository, search::SearchItemRepository,
            transfer::TransferItemRepository, update::UpdateItemRepository,
        },
    },
};

#[derive(Clone)]
pub struct SharedState {
    pub item_csv: ItemCsv,
    pub depreiation_csv: DepreiationCsv,
    pub transfer_item: TransferItem,
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
            item_csv: ItemCsv::new().await,
            depreiation_csv: DepreiationCsv::new().await,
            transfer_item: TransferItem::new().await,
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
