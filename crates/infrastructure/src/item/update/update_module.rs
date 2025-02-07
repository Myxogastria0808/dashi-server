use domain::{
    entity::data_type::{meilisearch::MeilisearchData, update_item::UpdateItemData},
    value_object::error::{critical_incident, update_item::UpdateItemError},
};
use entity::{
    item::{self, Entity as Item},
    label::Entity as Label,
};
use meilisearch_sdk::client::Client;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

pub(super) async fn update(
    rdb: DatabaseConnection,
    meilisearch: Client,
    update_item_data: UpdateItemData,
) -> Result<(), UpdateItemError> {
    ////* validation *////
    //* validation of id is not 1 *//
    // validation of id is not 1 in Item Table
    if update_item_data.id == 1 {
        // If id is 1
        return Err(UpdateItemError::CannotupdateRootItemError);
    }

    //* validation of name is not empty *//
    if update_item_data.name.chars().count() == 0 {
        return Err(UpdateItemError::ItemNameEmptyError);
    }

    //* validation of id is exist *//
    // validation of id is exist in Item Table
    let item_model = match Item::find_by_id(update_item_data.id).all(&rdb).await {
        Ok(item_models) => {
            if item_models.len() > 1 {
                // If multiple ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(UpdateItemError::IdConflictInItemTableError);
            }
            if item_models.is_empty() {
                // If id does not exist
                return Err(UpdateItemError::IdNotFoundInItemTableError);
            }
            item_models[0].clone()
        }
        Err(e) => return Err(UpdateItemError::RDBError(e)),
    };
    // validation of id is exist in MeiliSearch
    let filter_query = &format!(r#"id = "{}""#, update_item_data.id);
    let meilisearch_item: Vec<MeilisearchData> = meilisearch
        .index("item")
        .search()
        .with_query(&update_item_data.id.to_string())
        .with_filter(filter_query)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    if meilisearch_item.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::conflict_error().await;
        return Err(UpdateItemError::IdConflictInMeiliSerachError);
    }
    if meilisearch_item.is_empty() {
        // If visible_id does not exist
        return Err(UpdateItemError::IdNotFoundInMeiliSearchError);
    }
    //drop filter_query and meilisearch_item
    let _ = filter_query;
    let _ = meilisearch_item;

    //* validation of visible_id is exist in Label Table *//
    let label_model = match Label::find_by_id(update_item_data.visible_id.to_owned())
        .one(&rdb)
        .await?
    {
        Some(label_model) => label_model,
        None => return Err(UpdateItemError::LabelNotFoundError),
    };

    //* validation of not conflict color *//
    // if color is not empty and color is not same as before
    if update_item_data.color.chars().count() != 0 || update_item_data.color != item_model.color {
        // validation of not conflict color in Item Table
        match Item::find()
            .filter(item::Column::Color.eq(update_item_data.color.to_owned()))
            .all(&rdb)
            .await
        {
            Ok(item_models) => {
                if !item_models.is_empty() {
                    if item_models.len() > 1 {
                        // If multiple visible_ids already exist
                        //* critical incident *//
                        critical_incident::conflict_error().await;
                        return Err(UpdateItemError::ColorPatternConflictInItemTableError);
                    }
                    return Err(UpdateItemError::ColorPatternExistInItemTableError);
                }
            }
            Err(e) => return Err(UpdateItemError::RDBError(e)),
        };
        // validation of not conflict color in MeiliSearch
        let filter_query = &format!(r#"color = "{}""#, update_item_data.color.to_owned());
        let meilisearch_item: Vec<MeilisearchData> = meilisearch
            .index("item")
            .search()
            .with_query(&update_item_data.color.to_owned())
            .with_filter(filter_query)
            .execute()
            .await?
            .hits
            .into_iter()
            .map(|item| item.result)
            .collect();
        if !meilisearch_item.is_empty() {
            if meilisearch_item.len() > 1 {
                // If multiple visible_ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(UpdateItemError::ColorPatternConflictInMeiliSearchError);
            }
            return Err(UpdateItemError::ColorPatternExistInMeiliSearcheError);
        }
    }

    ////* operation *////
    //* update Item Table *//
    let mut active_item_model = item_model.clone().into_active_model();

    active_item_model.visible_id = Set(item_model.visible_id.to_owned());
    active_item_model.name = Set(update_item_data.name.to_owned());
    active_item_model.product_number = Set(update_item_data.product_number.to_owned());
    active_item_model.description = Set(update_item_data.description.to_owned());
    active_item_model.purchase_year = Set(update_item_data.purchase_year);
    active_item_model.purchase_price = Set(update_item_data.purchase_price);
    active_item_model.durability = Set(update_item_data.durability);
    active_item_model.is_depreciation = Set(update_item_data.is_depreciation);
    active_item_model.connector = Set(serde_json::json!(update_item_data.connector));
    active_item_model.is_rent = Set(update_item_data.is_rent);
    active_item_model.color = Set(update_item_data.color.to_owned());
    active_item_model.updated_at = Set(chrono::Utc::now().naive_local());

    let updated_item_model = match active_item_model.update(&rdb).await {
        Ok(item_model) => {
            tracing::info!("Updated to Item Table: {:?}", item_model);
            item_model
        }
        Err(e) => {
            tracing::error!("Failed to insert item.");
            tracing::error!("{}", e.to_string());
            return Err(UpdateItemError::RDBError(e));
        }
    };

    //* update MeiliSearch *//
    let meilisearch_model: MeilisearchData = MeilisearchData {
        id: updated_item_model.id,
        visible_id: updated_item_model.visible_id.to_owned(),
        record: label_model.record,
        name: updated_item_model.name.to_owned(),
        product_number: updated_item_model.product_number.to_owned(),
        description: updated_item_model.description.to_owned(),
        purchase_year: updated_item_model.purchase_year,
        purchase_price: updated_item_model.purchase_price,
        durability: updated_item_model.durability,
        is_depreciation: updated_item_model.is_depreciation,
        connector: update_item_data.connector.to_owned(),
        is_rent: updated_item_model.is_rent,
        color: updated_item_model.color.to_owned(),
        created_at: updated_item_model.created_at.to_owned(),
        updated_at: updated_item_model.updated_at.to_owned(),
    };
    match meilisearch
        .index("item")
        .add_documents(&[meilisearch_model], Some("id"))
        .await
    {
        Ok(insert_meilisearch_item_model) => {
            tracing::info!("MeiliSearch result of item.");
            tracing::info!("{:#?}", insert_meilisearch_item_model);
        }
        Err(e) => {
            tracing::error!("Failed to insert meilisearch.");
            // try rollback
            rollback_rdb(&rdb, item_model).await?;
            return Err(UpdateItemError::MeiliSearchError(e));
        }
    }

    Ok(())
}

async fn rollback_rdb(
    rdb: &DatabaseConnection,
    item_model: item::Model,
) -> Result<(), UpdateItemError> {
    let mut active_item_model = item_model.clone().into_active_model();

    active_item_model.visible_id = Set(item_model.visible_id.to_owned());
    active_item_model.name = Set(item_model.name.to_owned());
    active_item_model.product_number = Set(item_model.product_number.to_owned());
    active_item_model.description = Set(item_model.description.to_owned());
    active_item_model.purchase_year = Set(item_model.purchase_year);
    active_item_model.purchase_price = Set(item_model.purchase_price);
    active_item_model.durability = Set(item_model.durability);
    active_item_model.is_depreciation = Set(item_model.is_depreciation);
    active_item_model.connector = Set(item_model.connector);
    active_item_model.is_rent = Set(item_model.is_rent);
    active_item_model.color = Set(item_model.color.to_owned());
    active_item_model.created_at = Set(item_model.created_at.to_owned());
    active_item_model.updated_at = Set(item_model.updated_at.to_owned());

    let item_model = match active_item_model.update(rdb).await {
        Ok(item_model) => item_model,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback updated item in Item Table (rollback updated item infomation)."
            );
            tracing::warn!("Rollbaack Summary");
            tracing::warn!("RDB: Failed");
            return Err(UpdateItemError::RDBError(e));
        }
    };

    tracing::info!("Rollbacked registed item in Item Table (rollback updated item infomation).");
    tracing::debug!("{:#?}", item_model);
    tracing::warn!("Rollback Summary");
    tracing::warn!("RDB: Success");
    Ok(())
}
