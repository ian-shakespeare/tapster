use std::env;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tapster_api::AppState;

#[tokio::main]
async fn main() {
    let _ = dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL environment variable");
    let pool = match PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            panic!("failed to connect to database: {:?}", e);
        }
    };

    let s3_key =
        env::var("S3_ACCESS_KEY_ID").expect("missing S3_ACCESS_KEY_ID environment variable");
    let s3_secret = env::var("S3_SECRET_ACCESS_KEY")
        .expect("missing S3_SECRET_ACCESS_KEY environment variable");
    let s3_url = env::var("S3_URL").expect("missing S3_URL environment variable");
    let s3_region = env::var("S3_REGION").unwrap_or("us-east-1".to_string());
    let cred = Credentials::new(s3_key, s3_secret, None, None, "loaded-from-custom-env");
    let config = aws_sdk_s3::config::Builder::new()
        .behavior_version(BehaviorVersion::latest())
        .endpoint_url(s3_url)
        .credentials_provider(cred)
        .region(Region::new(s3_region))
        .force_path_style(true)
        .build();
    let client = aws_sdk_s3::Client::from_conf(config);

    if let Err(e) = client
        .head_bucket()
        .bucket(tapster_api::MEDIA_BUCKET)
        .send()
        .await
    {
        if e.into_service_error().is_not_found() {
            client
                .create_bucket()
                .bucket(tapster_api::MEDIA_BUCKET)
                .send()
                .await
                .expect("failed to create media s3 bucket");
        }
    }

    let signing_key = env::var("SIGNING_KEY").expect("missing SIGNING_KEY environment variable");

    let app = tapster_api::router(AppState::new(pool.clone(), client, signing_key));
    let addr = "0.0.0.0:8000";

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind tcp listener");

    println!("Listening on {addr}");
    axum::serve(listener, app)
        .await
        .expect("failed to serve listener");
}
