use crate::connection;
use crate::item::register::register;
use domain::{
    entity::data_type::register_item::RegisterItemData,
    repository::{connection::ConnectionRepository, item::register::RegisterItemRepository},
    value_object::error::AppError,
};
use serde::Deserialize;

pub mod delete;
pub mod read;
pub mod register;
pub mod update;

#[derive(Clone, Debug, Deserialize)]
pub struct RegisterItem {
    register_item_data: RegisterItemData,
}

impl RegisterItemRepository for RegisterItem {
    async fn new(register_item_data: RegisterItemData) -> Self {
        Self { register_item_data }
    }
    async fn register(&self) -> Result<(), AppError> {
        let connect_collection = connection::CollectConnection::new().await?;
        register(
            connect_collection.rdb,
            connect_collection.graphdb,
            connect_collection.meilisearch,
            self.register_item_data.to_owned(),
        )
        .await?;
        Ok(())
    }
}
