use crate::value_object::error::healthcheck::HealthCheckError;

pub trait HealthCheckRepository {
    fn healthcheck_graphdb(&self) -> impl Future<Output = Result<(), HealthCheckError>> + Send;
    fn healthcheck_meilisearch(&self) -> impl Future<Output = Result<(), HealthCheckError>> + Send;
    fn healthcheck_rdb(&self) -> impl Future<Output = Result<(), HealthCheckError>> + Send;
}
