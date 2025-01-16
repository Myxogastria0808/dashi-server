use domain::entity::data_type::meilisearch;
use entity::{
    item::{self, Entity as Item},
    label::{self, Entity as Label},
};
use infrastructure::connection;
use neo4rs::{Node, query};
use sea_orm::{self, EntityTrait, Set};
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Connect rdb
    let rdb = connection::rdb::connect_postgres()
        .await
        .expect("Failed to connect to Postgres");
    // Connect graphdb
    let graphdb = connection::graphdb::connect_neo4j().await;
    // Connect meilisearch
    let meilisearch = connection::meilisearch::connect_meilisearch().await;
    // Connect r2
    let r2 = connection::object_strage::connect_r2().await;
    // Get r2 url
    // let r2_url = connection::object_strage::get_r2_url().await;

    // Add rdb data //
    // Insert data into the Label table
    let label_model: label::ActiveModel = label::ActiveModel {
        visible_id: Set("0000".to_string()),
        record: Set(label::Record::Nothing),
        ..Default::default()
    };
    let inserted_label_model = Label::insert(label_model)
        .exec(&rdb)
        .await
        .expect("Failed to insert label");
    println!(
        "[INFO]: Inserted to Label Table: {:?}",
        inserted_label_model
    );
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
    let inserted_item_model = Item::insert(item_model)
        .exec(&rdb)
        .await
        .expect("Failed to insert item");
    println!("[INFO]: Inserted to Item Table: {:?}", inserted_item_model);

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
    let insert_meilisearch_item_model = meilisearch
        .index("item")
        .add_documents(&[meilisearch_model], Some("id"))
        .await
        .unwrap();
    println!(
        "[INFO]: MeiliSearch result of item\n{:#?}",
        insert_meilisearch_item_model
    );

    // Add graphdb data //
    graphdb
        .run(query("CREATE (item:Item {id: $id})").param("id", 1))
        .await
        .expect("Failed to create item node");
    // get node
    let mut insert_graphdb_item_node = graphdb
        .execute(query("MATCH node=(item:Item {id: $id}) RETURN node").param("id", 1))
        .await
        .unwrap();
    // parse node
    loop {
        let item = insert_graphdb_item_node.next().await.unwrap();
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node = row.get::<Node>("node").unwrap();
        let id = node.get::<i64>("id").unwrap();
        println!("[INFO]: GraphDb result of item\n{:#?}", id);
    }

    // Add r2 data //
    //open file
    let mut file = File::open("./image/tsukuba.webp")
        .await
        .expect("Failed to open file");
    //read binary
    let mut buffer = Vec::new();
    //close file
    file.read_to_end(&mut buffer)
        .await
        .expect("Failed to close file");
    //upload file
    r2.upload(
        "1.webp",
        &buffer[..],
        Some("max-age=60"),
        Some("image/webp"),
    )
    .await;
    //get file
    let result = r2.get("1.webp").await.expect("Failed to upload file");
    println!("[INFO]: R2 result of file\n{:#?}", result);

    // Close rdb
    let _ = rdb.close().await;

    // Finish!
    println!("[INFO]: Finish!");
}
