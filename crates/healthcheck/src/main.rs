use domain::repository::healthcheck::HealthCheckRepository;
use infrastructure::healthcheck;
use init::initializer;

mod init;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    //* Check *//
    let health_check = healthcheck::HealthCheck::new().await;
    // health check and initialize (if not initialized)
    match health_check.healthcheck().await {
        Ok(_) => {
            // already initialized
            tracing::info!("HealthCheck and Initialize ware passed.");
        }
        Err(_) => {
            // not initialized
            // initialize
            initializer().await;
        }
    };
}
