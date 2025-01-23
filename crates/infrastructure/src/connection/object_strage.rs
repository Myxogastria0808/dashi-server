use cf_r2_sdk::{builder::Builder, operator::Operator};
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use std::env;

pub async fn connect_r2() -> Operator {
    // Set environment variables
    // Declaration and initialization of static variable
    static CLOUDFLARE_R2_BUCKET_NAME: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_R2_URI_ENDPOINT: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID: OnceCell<String> = OnceCell::new();
    static CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = CLOUDFLARE_R2_BUCKET_NAME.set(
        env::var("CLOUDFLARE_R2_BUCKET_NAME")
            .expect("CLOUDFLARE_R2_BUCKET_NAME not found in .env file."),
    );
    let _ = CLOUDFLARE_R2_URI_ENDPOINT.set(
        env::var("CLOUDFLARE_R2_URI_ENDPOINT")
            .expect("CLOUDFLARE_R2_URI_ENDPOINT not found in .env file."),
    );
    let _ = CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID.set(
        env::var("CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID")
            .expect("CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID not found in .env file."),
    );
    let _ = CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY.set(
        env::var("CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY")
            .expect("CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY not found in .env file."),
    );
    //インスタンスの作成
    Builder::new()
        .set_bucket_name(
            CLOUDFLARE_R2_BUCKET_NAME
                .get()
                .expect("Failed to get CLOUDFLARE_R2_BUCKET_NAME.")
                .clone(),
        )
        .set_endpoint(
            CLOUDFLARE_R2_URI_ENDPOINT
                .get()
                .expect("Failed to get CLOUDFLARE_R2_URI_ENDPOINT.")
                .clone(),
        )
        .set_access_key_id(
            CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID
                .get()
                .expect("Failed to get CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID.")
                .clone(),
        )
        .set_secret_access_key(
            CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY
                .get()
                .expect("Failed to get CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY.")
                .clone(),
        )
        .create_client()
}
