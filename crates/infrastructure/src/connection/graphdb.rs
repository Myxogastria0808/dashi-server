use dotenvy::dotenv;
use neo4rs::Graph;
use once_cell::sync::OnceCell;
use std::env;

pub async fn connect_neo4j() -> Graph {
    // Set environment variables
    // Declaration and initialization of static variable
    static NEO4J_BOLT_PORT: OnceCell<String> = OnceCell::new();
    static NEO4J_USER: OnceCell<String> = OnceCell::new();
    static NEO4J_PASSWORD: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = NEO4J_BOLT_PORT
        .set(env::var("NEO4J_BOLT_PORT").expect("NEO4J_BOLT_PORT not found in .env file."));
    let _ = NEO4J_USER.set(env::var("NEO4J_USER").expect("NEO4J_USER not found in .env file."));
    let _ = NEO4J_PASSWORD
        .set(env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD not found in .env file."));
    //インスタンスの作成
    Graph::new(
        format!(
            // "neo4j:{}",
            "localhost:{}",
            NEO4J_BOLT_PORT
                .get()
                .expect("Failed to get NEO4J_BOLT_PORT")
        ),
        NEO4J_USER.get().expect("Failed to get NEO4J_USER"),
        NEO4J_PASSWORD.get().expect("Failed to get NEO4J_PASSWORD"),
    )
    .await
    .expect("Cannot connect to Neo4j")
}
