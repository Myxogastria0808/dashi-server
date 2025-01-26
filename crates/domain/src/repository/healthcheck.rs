use crate::value_object::error::healthcheck::HealthCheckError;
use async_std::future::Future;

pub trait HealthCheckRepository {
    fn new() -> Self;
    fn healthcheck(&self) -> impl Future<Output = Result<(), HealthCheckError>> + Send;
}
