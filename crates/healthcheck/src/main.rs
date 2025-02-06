use domain::repository::healthcheck::HealthCheckRepository;
use infrastructure::healthcheck;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    // HealthCheck
    let health_check = healthcheck::HealthCheck::new().await;
    let result = health_check.healthcheck().await;
}
