use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};

use auth::*;
use error::*;
use handlers::*;

mod auth;
mod error;
mod handlers;
mod model;

pub const MEDIA_BUCKET: &str = "tapsters-media";

type Result<T> = std::result::Result<T, crate::Error>;

pub struct AppState {
    db: Pool<Postgres>,
    s3: aws_sdk_s3::Client,
    signing_key: String,
}

impl AppState {
    pub fn new(db: Pool<Postgres>, s3: aws_sdk_s3::Client, signing_key: String) -> Self {
        Self {
            db,
            s3,
            signing_key,
        }
    }
}

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .route("/api/units", get(list_units_handler))
        .route("/api/register", post(register_user_handler))
        .route("/api/sign-in", post(sign_in_handler))
        .route("/api/bars", post(create_bar_handler).get(list_bars_handler))
        .route("/api/bars/{bar_id}", get(get_bar_handler))
        .route("/api/media", post(create_media_handler))
        .route("/api/media/{media_id}", get(get_media_handler))
        .route(
            "/api/ingredients",
            post(create_ingredient_handler).get(list_ingredients_handler),
        )
        .route(
            "/api/ingredients/{ingredient_id}",
            get(get_ingredient_handler),
        )
        .route(
            "/api/ingredients/{ingredient_id}/ingredients",
            get(get_ingredient_ingredients_handler),
        )
        .with_state(Arc::new(app_state))
}
