use domain::{
    entity::data_type::transfer_item::TransferItemData,
    value_object::error::{critical_incident, transfer_item::TransferItemError},
};
use entity::{
    item::{self, Entity as Item},
    label::Entity as Label,
};
use neo4rs::{query, Graph};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

pub(super) async fn transfer(
    rdb: DatabaseConnection,
    graphdb: Graph,
    transfer_item_data_inputs: Vec<TransferItemData>,
) -> Result<(), TransferItemError> {
    Ok(())
}
