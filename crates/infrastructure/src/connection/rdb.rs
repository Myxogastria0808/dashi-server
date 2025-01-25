use domain::value_object::error::connection::ConnectionError;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use sea_orm::{self, Database, DatabaseConnection};
use std::env;

pub(super) async fn connect_postgres() -> Result<DatabaseConnection, ConnectionError> {
    // Declaration and initialization of static variable
    static POSTGRES_USER: OnceCell<String> = OnceCell::new();
    static POSTGRES_PASSWORD: OnceCell<String> = OnceCell::new();
    static POSTGRES_PORT: OnceCell<String> = OnceCell::new();
    static POSTGRES_DB: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv()?;
    // set Object value
    let _ = POSTGRES_USER.set(env::var("POSTGRES_USER")?);
    let _ = POSTGRES_PASSWORD.set(env::var("POSTGRES_PASSWORD")?);
    let _ = POSTGRES_PORT.set(env::var("POSTGRES_PORT")?);
    let _ = POSTGRES_DB.set(env::var("POSTGRES_DB")?);
    // connnect database
    Ok(Database::connect(format!(
        // "postgres://{}:{}@postgres:{}/{}",
        "postgres://{}:{}@localhost:{}/{}",
        POSTGRES_USER
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "POSTGRES_USER".to_string(),
            ))?,
        POSTGRES_PASSWORD
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "POSTGRES_PASSWORD".to_string(),
            ))?,
        POSTGRES_PORT
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "POSTGRES_PORT".to_string(),
            ))?,
        POSTGRES_DB
            .get()
            .ok_or(ConnectionError::DotEnvVarNotFountError(
                "POSTGRES_DB".to_string(),
            ))?,
    ))
    .await?)
}
