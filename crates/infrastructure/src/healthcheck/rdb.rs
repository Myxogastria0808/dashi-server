use domain::value_object::error::healthcheck::HealthCheckError;
use entity::{
    item::{self, Entity as Item},
    label::{self, Entity as Label, Record},
};
use sea_orm::{DatabaseConnection, EntityTrait};

pub(super) async fn healthcheck_rdb(rdb: DatabaseConnection) -> Result<(), HealthCheckError> {
    //* test *//
    rdb.ping().await?;

    //* check *//
    let root_label = match Label::find_by_id("0000".to_string()).one(&rdb).await {
        Ok(label_model) => match label_model {
            Some(label_model) => label_model,
            None => return Err(HealthCheckError::RootItemNotFoundError),
        },
        Err(e) => return Err(HealthCheckError::RDBError(e)),
    };
    let correct_root_label = label::Model {
        visible_id: "0000".to_string(),
        is_max: root_label.is_max,
        record: Record::Nothing,
    };

    if root_label != correct_root_label {
        return Err(HealthCheckError::IncompatibleInLabelTableError);
    }

    let root_item = match Item::find_by_id(1).one(&rdb).await {
        Ok(item_model) => match item_model {
            Some(item_model) => item_model,
            None => return Err(HealthCheckError::RootItemNotFoundError),
        },
        Err(e) => return Err(HealthCheckError::RDBError(e)),
    };
    let connector: Vec<String> = Vec::new();
    let correct_root_item = item::Model {
        id: 1,
        visible_id: "0000".to_string(),
        name: "筑波大学".to_string(),
        product_number: "".to_string(),
        description: "根の物品です。".to_string(),
        purchase_year: None,
        purchase_price: None,
        durability: None,
        is_depreciation: false,
        connector: serde_json::json!(connector),
        is_rent: false,
        color: "".to_string(),
        created_at: root_item.created_at,
        updated_at: root_item.updated_at,
    };

    if root_item != correct_root_item {
        return Err(HealthCheckError::IncompatibleInItemTableError);
    }

    tracing::info!("RDB is healthy.");
    Ok(())
}
