use std::sync::{Arc, RwLock};

use meilisearch_sdk::client::Client;
use neo4rs::Graph;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct SharedState {
    pub graph_db: Graph,
    pub meilisearch: Client,
    pub rdb: DatabaseConnection,
}

pub type RwLockSharedState = Arc<RwLock<SharedState>>;
