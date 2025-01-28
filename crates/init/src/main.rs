use std::fmt::DebugStruct;

use domain::{entity::data_type::meilisearch, repository::connection::ConnectionRepository};
use entity::{
    item::{self, Entity as Item},
    label::{self, Entity as Label},
};
use infrastructure::connection;
use neo4rs::{query, Node};
use sea_orm::{self, EntityTrait, Set};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    // Connect rdb
    let rdb = match connection::CollectConnection::connect_rdb().await {
        Ok(rdb) => rdb,
        Err(e) => {
            tracing::error!("Failed to connect to PostgreSQL.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };
    // Connect graphdb
    let graphdb = match connection::CollectConnection::connect_graphdb().await {
        Ok(graphdb) => graphdb,
        Err(e) => {
            tracing::error!("Failed to connect to Neo4j.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };
    // Connect meilisearch
    let meilisearch = match connection::CollectConnection::connect_meilisearch().await {
        Ok(meilisearch) => meilisearch,
        Err(e) => {
            tracing::error!("Failed to connect to Meilisearch.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };
    // Connect r2
    let r2 = match connection::CollectConnection::connect_object_strage().await {
        Ok(r2) => r2,
        Err(e) => {
            tracing::error!("Failed to connect to R2.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };

    // Add rdb data //
    // Insert data into the Label table
    let label_model: label::ActiveModel = label::ActiveModel {
        visible_id: Set("0000".to_string()),
        is_max: Set(true),
        record: Set(label::Record::Nothing),
    };
    let inserted_label_model = Label::insert(label_model).exec(&rdb).await;
    match inserted_label_model {
        Ok(label_model) => {
            tracing::info!("Inserted to Label Table: {:?}", label_model);
        }
        Err(e) => {
            tracing::error!("Failed to insert label.");
            tracing::error!("{}", e.to_string());
            return;
        }
    }
    // Insert data into the Item table
    let root_item_connector: Vec<String> = Vec::new();
    let item_model: item::ActiveModel = item::ActiveModel {
        visible_id: Set("0000".to_string()),
        is_waste: Set(false),
        name: Set("筑波大学".to_string()),
        product_number: Set("".to_string()),
        description: Set("ルートの物品です。".to_string()),
        is_depreciation: Set(false),
        connector: Set(serde_json::json!(root_item_connector)),
        is_rent: Set(false),
        color: Set("".to_string()),
        created_at: Set(chrono::Utc::now().naive_local()),
        updated_at: Set(chrono::Utc::now().naive_local()),
        ..Default::default()
    };
    match Item::insert(item_model).exec(&rdb).await {
        Ok(item_model) => {
            tracing::info!("Inserted to Item Table: {:?}", item_model);
        }
        Err(e) => {
            tracing::error!("Failed to insert item.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };

    // Add meilisearch data //
    let meilisearch_model: meilisearch::MeilisearchData = meilisearch::MeilisearchData {
        id: 1,
        visible_id: "0000".to_string(),
        record: label::Record::Nothing,
        is_waste: false,
        name: "筑波大学".to_string(),
        product_number: "".to_string(),
        description: "ルートの物品です。".to_string(),
        purchase_year: None,
        purchase_price: None,
        durability: None,
        is_depreciation: false,
        connector: root_item_connector,
        is_rent: false,
        color: "".to_string(),
        created_at: chrono::Utc::now().naive_local(),
        updated_at: chrono::Utc::now().naive_local(),
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
            tracing::error!("{}", e.to_string());
            return;
        }
    }
    // set as filterable
    match meilisearch
        .index("item")
        .set_filterable_attributes([
            "id",
            "visible_id",
            "record",
            "is_waste",
            "name",
            "product_number",
            "description",
            "purchase_year",
            "purchase_price",
            "durability",
            "is_depreciation",
            "connector",
            "is_rent",
            "color",
            "created_at",
            "updated_at",
        ])
        .await
    {
        Ok(task) => {
            tracing::info!("Set filterable attributes result.");
            tracing::info!("{:#?}", task);
        }
        Err(e) => {
            tracing::error!("Failed to set filterable attributes.");
            tracing::error!("{}", e.to_string());
            return;
        }
    }
    // get filterable attributes
    match meilisearch.index("item").get_filterable_attributes().await {
        Ok(filterable_attributes) => {
            tracing::info!("Get filterable attributes result.");
            tracing::info!("{:#?}", filterable_attributes);
        }
        Err(e) => {
            tracing::error!("Failed to get filterable attributes.");
            tracing::error!("{}", e.to_string());
            return;
        }
    }

    // Add graphdb data //
    match graphdb
        .to_owned()
        .run(query("CREATE (item:Item {id: $id})").param("id", 1))
        .await
    {
        Ok(graphdb) => graphdb,
        Err(e) => {
            tracing::error!("Failed to create item node.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };
    // get node
    let mut insert_graphdb_item_node = match graphdb
        .execute(query("MATCH (item:Item {id: $id}) RETURN item").param("id", 1))
        .await
    {
        Ok(graphdb) => graphdb,
        Err(e) => {
            tracing::error!("Failed to get item node.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };
    // parse node
    loop {
        let item = match insert_graphdb_item_node.next().await {
            Ok(item) => item,
            Err(e) => {
                tracing::error!("Failed to get item.");
                tracing::error!("{}", e.to_string());
                return;
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("item") {
            Ok(node) => node,
            Err(e) => {
                tracing::error!("Failed to get node.");
                tracing::error!("{}", e.to_string());
                return;
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("Failed to get id.");
                tracing::error!("{}", e.to_string());
                return;
            }
        };
        tracing::info!("GraphDB result of item.");
        tracing::info!("node: {:#?}", id);
    }

    // Add r2 data //
    match r2
        .upload_file("1.webp", "image/webp", "./crates/init/image/tsukuba.webp")
        .await
    {
        Ok(_) => {
            tracing::info!("Uploaded image file.");
        }
        Err(e) => {
            tracing::error!("Failed to upload file.");
            tracing::error!("{:?}", e);
            return;
        }
    };

    // Close rdb
    let _ = rdb.close().await;

    // Finish!
    tracing::info!("Finish!");
}
