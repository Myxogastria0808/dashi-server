use domain::value_object::error::healthcheck::HealthCheckError;
use sea_orm::DatabaseConnection;

pub(super) async fn healthcheck_rdb(rdb: DatabaseConnection) -> Result<(), HealthCheckError> {
    // test
    rdb.ping().await?;
    tracing::info!("RDB is healthy.");
    Ok(())
}
