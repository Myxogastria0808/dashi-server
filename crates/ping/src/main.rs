use domain::repository::connection::ConnectionRepository;
use infrastructure::connection;
use neo4rs::query;

#[tokio::main]
async fn main() {
    // tracing
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
    //* ping GraphDB *//
    // get (item:Item {id: 1}) test
    match graphdb
        .execute(query("MATCH (item:Item {id: $id}) RETURN item").param("id", 1))
        .await
    {
        Ok(_) => {
            tracing::info!("GraphDB is healthy.");
        }
        Err(e) => {
            tracing::error!("Failed to ping GraphDB.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };
    //* ping MeiliSearch *//
    match meilisearch.health().await {
        Ok(_) => {
            tracing::info!("MeiliSearch is healthy.");
        }
        Err(e) => {
            tracing::error!("Failed to ping MeiliSearch.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };
    //* ping RDB *//
    match rdb.ping().await {
        Ok(_) => {
            tracing::info!("RDB is healthy.");
        }
        Err(e) => {
            tracing::error!("Failed to ping RDB.");
            tracing::error!("{}", e.to_string());
            return;
        }
    };
}
