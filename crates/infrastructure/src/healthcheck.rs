use domain::{
    repository::healthcheck::HealthCheckRepository,
    value_object::error::healthcheck::HealthCheckError,
};
use graphdb::healthcheck_graphdb;
use meilisearch::healthcheck_meilisearch;
use meilisearch_sdk::client::Client;
use neo4rs::Graph;
use rdb::healthcheck_rdb;
use sea_orm::DatabaseConnection;

pub mod graphdb;
pub mod meilisearch;
pub mod rdb;

#[derive(Clone)]
pub struct HealthCheck {
    pub graphdb: Graph,
    pub meilisearch: Client,
    pub rdb: DatabaseConnection,
}

impl HealthCheckRepository for HealthCheck {
    async fn healthcheck_graphdb(&self) -> Result<(), HealthCheckError> {
        healthcheck_graphdb(self.graphdb.to_owned()).await
    }
    async fn healthcheck_meilisearch(&self) -> Result<(), HealthCheckError> {
        healthcheck_meilisearch(self.meilisearch.to_owned()).await
    }
    async fn healthcheck_rdb(&self) -> Result<(), HealthCheckError> {
        healthcheck_rdb(self.rdb.to_owned()).await
    }
}
