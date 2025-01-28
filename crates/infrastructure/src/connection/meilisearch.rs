use domain::value_object::error::connection::ConnectionError;
use dotenvy::dotenv;
use meilisearch_sdk::client::Client;
use once_cell::sync::OnceCell;
use std::env;

pub(super) async fn connect_meilisearch() -> Result<Client, ConnectionError> {
    // Set environment variables
    // Declaration and initialization of static variable
    static MEILI_PORT: OnceCell<String> = OnceCell::new();
    static MEILI_MASTER_KEY: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv()?;
    // set Object value
    let _ = MEILI_PORT.set(env::var("MEILI_PORT")?);
    let _ = MEILI_MASTER_KEY.set(env::var("MEILI_MASTER_KEY")?);
    // create Client instance
    Ok(Client::new(
        format!(
            // "http://meilisearch:{}",
            "http://localhost:{}",
            MEILI_PORT
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "MEILI_PORT".to_string(),
                ))?
        ),
        Some(
            MEILI_MASTER_KEY
                .get()
                .ok_or(ConnectionError::DotEnvVarNotFountError(
                    "MEILI_MASTER_KEY".to_string(),
                ))?,
        ),
    )?)
}
