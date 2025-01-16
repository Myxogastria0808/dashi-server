use crate::connection::{
    graphdb::connect_neo4j, meilisearch::connect_meilisearch, rdb::connect_postgres,
};
use neo4rs::query;

pub async fn healthcheck() -> Result<(), Box<dyn std::error::Error>> {
    //RDB
    let rdb: sea_orm::DatabaseConnection = connect_postgres().await.unwrap();
    rdb.ping().await?;
    //GraphDB
    let graphdb: neo4rs::Graph = connect_neo4j().await;
    let _ = graphdb
        .execute(query("MATCH (item:Item {id: $id}) RETURN item").param("id", 1))
        .await
        .unwrap();
    //Meilisearch
    let meilisearch: meilisearch_sdk::client::Client = connect_meilisearch().await;
    meilisearch.health().await?;
    Ok(())
}
