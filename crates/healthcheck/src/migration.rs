use std::env;

use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use tokio::process::Command;

pub(super) async fn migration() {
    // Migration
    // source /app/.env
    let output = Command::new("bash")
        .arg("-c")
        .arg("source /app/.env")
        .output()
        .await
        .expect("Failed to migration.");
    println!("{:#?}", std::str::from_utf8(&output.stdout));

    // cargo run --manifest-path /app/migration/Cargo.toml -- refresh -u postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{POSTGRES_HOST}:{POSTGRES_PORT}/{POSTGRES_DB}
    // Declaration and initialization of static variable
    static POSTGRES_USER: OnceCell<String> = OnceCell::new();
    static POSTGRES_PASSWORD: OnceCell<String> = OnceCell::new();
    static POSTGRES_PORT: OnceCell<String> = OnceCell::new();
    static POSTGRES_DB: OnceCell<String> = OnceCell::new();
    static POSTGRES_HOST: OnceCell<String> = OnceCell::new();
    // load .env file
    match dotenv() {
        Ok(_) => {}
        Err(_) => {
            tracing::error!("Failed to load .env file.");
            return;
        }
    }
    // set Object value
    match env::var("POSTGRES_USER") {
        Ok(postgres_user) => {
            let _ = POSTGRES_USER.set(postgres_user);
        }
        Err(_) => {
            tracing::error!("Failed to get POSTGRES_USER.");
            return;
        }
    };
    match env::var("POSTGRES_PASSWORD") {
        Ok(postgres_password) => {
            let _ = POSTGRES_PASSWORD.set(postgres_password);
        }
        Err(_) => {
            tracing::error!("Failed to get POSTGRES_PASSWORD.");
            return;
        }
    }
    match env::var("POSTGRES_PORT") {
        Ok(postgres_port) => {
            let _ = POSTGRES_PORT.set(postgres_port);
        }
        Err(_) => {
            tracing::error!("Failed to get POSTGRES_PORT.");
            return;
        }
    }
    match env::var("POSTGRES_DB") {
        Ok(postgres_db) => {
            let _ = POSTGRES_DB.set(postgres_db);
        }
        Err(_) => {
            tracing::error!("Failed to get POSTGRES_DB.");
            return;
        }
    }
    match env::var("POSTGRES_HOST") {
        Ok(postgres_host) => {
            let _ = POSTGRES_HOST.set(postgres_host);
        }
        Err(_) => {
            tracing::error!("Failed to get POSTGRES_HOST.");
            return;
        }
    }
    let command = format!(
        "cargo run --manifest-path /app/migration/Cargo.toml -- refresh -u postgres://{}:{}@{}:{}/{}",
        POSTGRES_USER.get().unwrap(),
        POSTGRES_PASSWORD.get().unwrap(),
        POSTGRES_HOST.get().unwrap(),
        POSTGRES_PORT.get().unwrap(),
        POSTGRES_DB.get().unwrap()
    );
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .await
        .expect("Failed to migration.");
    println!("{:#?}", std::str::from_utf8(&output.stdout));
}
