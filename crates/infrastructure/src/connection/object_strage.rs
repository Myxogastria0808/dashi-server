use cloudflare_r2_rs::r2::R2Manager;
use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use std::env;

pub async fn connect_r2() -> R2Manager {
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
    R2Manager::new(
        //Bucket Name
        CLOUDFLARE_R2_BUCKET_NAME
            .get()
            .expect("Failed to get CLOUDFLARE_R2_BUCKET_NAME"),
        //Cloudflare URI endpoint
        CLOUDFLARE_R2_URI_ENDPOINT
            .get()
            .expect("Failed to get CLOUDFLARE_R2_URI_ENDPOINT"),
        //API Token's Access Key ID
        CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID
            .get()
            .expect("Failed to get CLOUDFLARE_R2_API_TOKENS_ACCESS_KEY_ID"),
        //API Token's Secret Access Key
        CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY
            .get()
            .expect("Failed to get CLOUDFLARE_R2_API_TOKENS_SECRET_ACCESS_KEY"),
    )
    .await
}

pub async fn get_r2_url() -> String {
    // Set environment variables
    // Declaration and initialization of static variable
    static CLOUDFLARE_R2_URI_ENDPOINT: OnceCell<String> = OnceCell::new();
    // load .env file
    dotenv().expect(".env file not found.");
    // set Object value
    let _ = CLOUDFLARE_R2_URI_ENDPOINT.set(
        env::var("CLOUDFLARE_R2_URI_ENDPOINT")
            .expect("CLOUDFLARE_R2_URI_ENDPOINT not found in .env file."),
    );
    CLOUDFLARE_R2_URI_ENDPOINT
        .get()
        .expect("Failed to get CLOUDFLARE_R2_URI_ENDPOINT")
        .to_string()
}
