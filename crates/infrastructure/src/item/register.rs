use domain::{
    entity::data_type::{
        meilisearch::{self, MeilisearchData},
        register_item::RegisterItemData,
    },
    value_object::error::{critical_incident, register_item::RegisterItemError},
};
use entity::{
    item::{self, Entity as Item},
    label::Entity as Label,
};
use meilisearch_sdk::client::Client;
use neo4rs::{query, Graph};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

pub(super) async fn register(
    rdb: DatabaseConnection,
    graphdb: Graph,
    meilisearch: Client,
    register_item_data: RegisterItemData,
) -> Result<(), RegisterItemError> {
    //* validation *//
    // validation of name is not empty
    if register_item_data.name.chars().count() == 0 {
        return Err(RegisterItemError::ItemNameEmptyError);
    }
    // validation of visible_id is exist in Label Table
    let label_model = match Label::find_by_id(register_item_data.visible_id.to_owned())
        .one(&rdb)
        .await?
    {
        Some(label_model) => label_model,
        None => return Err(RegisterItemError::LabelNotFoundError),
    };
    // validation of visible_id is not exist in Item Table
    match Item::find()
        .filter(item::Column::VisibleId.eq(register_item_data.visible_id.to_owned()))
        .all(&rdb)
        .await
    {
        Ok(item_models) => {
            if !item_models.is_empty() {
                if item_models.len() > 1 {
                    // If multiple visible_ids already exist
                    //* critical incident *//
                    critical_incident::conflict_error().await;
                    return Err(RegisterItemError::VisibleIdConflictInItemTableError);
                }
                return Err(RegisterItemError::VisibleIdExistInItemTableError);
            }
        }
        Err(e) => return Err(RegisterItemError::RDBError(e)),
    }
    // calidation of visible_id is not exist in MeiliSearch
    let filter_query = &format!(
        r#"visible_id = "{}""#,
        register_item_data.visible_id.to_owned()
    );
    let meilisearch_item: Vec<MeilisearchData> = meilisearch
        .index("item")
        .search()
        .with_query(&register_item_data.visible_id.to_owned())
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
            return Err(RegisterItemError::VisibleIdConflictInMeiliSerachError);
        }
        return Err(RegisterItemError::VisibleIdExistInMeiliSerachError);
    }
    //drop filter_query and meilisearch_item
    let _ = filter_query;
    let _ = meilisearch_item;
    // validation of parent_viible_id is exist in Item Table.
    let parent_item_model = match Item::find()
        .filter(item::Column::VisibleId.eq(register_item_data.parent_visible_id.to_owned()))
        .filter(item::Column::IsWaste.eq(false))
        .all(&rdb)
        .await
    {
        Ok(item_models) => {
            if item_models.is_empty() {
                return Err(RegisterItemError::ParentVisibleIdNotFoundError);
            }
            if item_models.len() > 1 {
                // If multiple visible_ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(RegisterItemError::VisibleIdConflictInItemTableError);
            }
            item_models[0].to_owned()
        }
        Err(e) => return Err(RegisterItemError::RDBError(e)),
    };
    //TODO: 以下を実装
    // validation of parent_viible_id is exist in MeiliSearch.
    //TODO: 以下を実装
    // validation of parent_viible_id is exist in GraphDB.

    // validation of not conflict color
    if register_item_data.color.chars().count() != 0 {
        // validation of not conflict color in Item Table
        match Item::find()
            .filter(item::Column::Color.eq(register_item_data.color.to_owned()))
            .all(&rdb)
            .await
        {
            Ok(item_models) => {
                if !item_models.is_empty() {
                    if item_models.len() > 1 {
                        // If multiple visible_ids already exist
                        //* critical incident *//
                        critical_incident::conflict_error().await;
                        return Err(RegisterItemError::ColorPatternConflictInItemTableError);
                    }
                    return Err(RegisterItemError::ColorPatternExistInItemTableError);
                }
            }
            Err(e) => return Err(RegisterItemError::RDBError(e)),
        };
        // validation of not conflict color in MeiliSearch
        let filter_query = &format!(r#"color = "{}""#, register_item_data.color.to_owned());
        let meilisearch_item: Vec<MeilisearchData> = meilisearch
            .index("item")
            .search()
            .with_query(&register_item_data.color.to_owned())
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
                return Err(RegisterItemError::ColorPatternConflictInMeiliSearchError);
            }
            return Err(RegisterItemError::ColorPatternExistInMeiliSearcheError);
        }
    }

    //* operation *//
    // insert to RDB
    let item_model = item::ActiveModel {
        visible_id: Set(register_item_data.visible_id.to_owned()),
        is_waste: Set(false),
        name: Set(register_item_data.name.to_owned()),
        product_number: Set(register_item_data.product_number.to_owned()),
        description: Set(register_item_data.description.to_owned()),
        purchase_year: Set(register_item_data.purchase_year),
        purchase_price: Set(register_item_data.purchase_price),
        durability: Set(register_item_data.durability),
        is_depreciation: Set(register_item_data.is_depreciation),
        connector: Set(serde_json::json!(register_item_data.connector)),
        is_rent: Set(false),
        color: Set(register_item_data.color.to_owned()),
        created_at: Set(chrono::Utc::now().naive_local()),
        updated_at: Set(chrono::Utc::now().naive_local()),
        ..Default::default()
    };
    let inserted_item_model = match Item::insert(item_model).exec(&rdb).await {
        Ok(item_model) => {
            tracing::info!("Inserted to Item Table: {:?}", item_model);
            item_model
        }
        Err(e) => {
            tracing::error!("Failed to insert item.");
            tracing::error!("{}", e.to_string());
            return Err(RegisterItemError::RDBError(e));
        }
    };
    //get registered item
    let registered_item_model = match Item::find_by_id(inserted_item_model.last_insert_id)
        .one(&rdb)
        .await
    {
        Ok(item_model) => match item_model {
            Some(item_model) => item_model,
            None => return Err(RegisterItemError::RegisteredItemNotFoundError),
        },
        Err(e) => return Err(RegisterItemError::RDBError(e)),
    };

    // insert to meilisearch
    let meilisearch_model: MeilisearchData = MeilisearchData {
        id: registered_item_model.id,
        visible_id: registered_item_model.visible_id.to_owned(),
        record: label_model.record,
        is_waste: false,
        name: registered_item_model.name.to_owned(),
        product_number: registered_item_model.product_number.to_owned(),
        description: registered_item_model.description.to_owned(),
        purchase_year: registered_item_model.purchase_year,
        purchase_price: registered_item_model.purchase_price,
        durability: registered_item_model.durability,
        is_depreciation: registered_item_model.is_depreciation,
        connector: register_item_data.connector.to_owned(),
        is_rent: false,
        color: registered_item_model.color.to_owned(),
        created_at: registered_item_model.created_at.to_owned(),
        updated_at: registered_item_model.updated_at.to_owned(),
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
            rollback_rdb(&rdb, registered_item_model).await?;
            return Err(RegisterItemError::MeiliSearchError(e));
        }
    }

    // insert to GraphDB
    // create item node
    match graphdb
        .run(query("MATCH (parent:Item {id: $parent_id}) CREAT (child:Item {id: $child_id})-[relation:ItemTree]->(parent)")
        .param("parent_id", parent_item_model.id)
        .param("child_id", registered_item_model.id))
        .await
    {
        Ok(result) => {
            tracing::info!("Inserted to GraphDB");
            tracing::debug!("{:#?}", result);
        }
        Err(e) => {
            tracing::error!("Failed to insert item.");
            // try rollback
            rollback_rdb_meilisearch(&rdb, meilisearch, registered_item_model).await?;
            return Err(RegisterItemError::GraphDBError(e));
        }
    }

    Ok(())
}

async fn rollback_rdb_meilisearch(
    rdb: &DatabaseConnection,
    meilisearch: Client,
    registered_item_model: item::Model,
) -> Result<(), RegisterItemError> {
    // rollback RDB
    match rollback_rdb(rdb, registered_item_model.to_owned()).await {
        Ok(_) => {
            // Rollbacked RDB: Success
            // rollback MeiliSearch
            match rollback_meilisearch(meilisearch, registered_item_model.to_owned()).await {
                Ok(_) => {
                    // Rollbacked RDB: Success, Rollbacked MeiliSearch: Success
                }
                Err(e) => {
                    // Rollbacked RDB: Success, Rollbacked MeiliSearch: Failed
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // Rollbacked RDB: Failed
            // rollback MeiliSearch
            match rollback_meilisearch(meilisearch, registered_item_model.to_owned()).await {
                Ok(_) => {
                    // Rollbacked RDB: Failed, Rollbacked MeiliSearch: Success
                }
                Err(e) => {
                    // Rollbacked RDB: Failed, Rollbacked MeiliSearch: Failed
                    return Err(e);
                }
            }
            return Err(e);
        }
    }
    Ok(())
}

async fn rollback_rdb(
    rdb: &DatabaseConnection,
    registered_item_model: item::Model,
) -> Result<(), RegisterItemError> {
    let item_model = match registered_item_model.into_active_model().delete(rdb).await {
        Ok(result) => result,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback registed item in Item Table (delete registered item)."
            );
            tracing::debug!("Rollbaack Summary");
            tracing::debug!("RDB: Failed");
            return Err(RegisterItemError::RDBError(e));
        }
    };
    tracing::debug!("Rollbacked registed item in Item Table (delete registered item).");
    tracing::debug!("{:#?}", item_model);
    tracing::debug!("Rollback Summary");
    tracing::debug!("RDB: Success");
    Ok(())
}

async fn rollback_meilisearch(
    meilisearch: Client,
    registered_item_model: item::Model,
) -> Result<(), RegisterItemError> {
    let meilisearch_model = meilisearch
        .index("item")
        .delete_document(registered_item_model.id)
        .await;
    let meilisearch_model = match meilisearch_model {
        Ok(result) => result,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback registed item in MeiliSearch (delete registered item)."
            );
            tracing::debug!("Rollback Summary");
            tracing::debug!("MeiliSearch: Failed");
            return Err(RegisterItemError::MeiliSearchError(e));
        }
    };
    tracing::info!("Rollbacked registed item in MeiliSearch (delete registered item).");
    tracing::debug!("{:#?}", meilisearch_model);
    tracing::debug!("Rollack Summary");
    tracing::debug!("MeiliSearch: Success");
    Ok(())
}
