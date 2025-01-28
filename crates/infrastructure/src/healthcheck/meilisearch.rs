use domain::value_object::error::healthcheck::HealthCheckError;
use meilisearch_sdk::client::Client;

pub(super) async fn healthcheck_meilisearch(meilisearch: Client) -> Result<(), HealthCheckError> {
    //* test *//
    let _ = meilisearch.health().await?;
    tracing::info!("MeiliSearch is healthy.");
    Ok(())
}
