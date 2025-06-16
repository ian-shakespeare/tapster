use std::sync::Arc;

use axum::Router;
use sqlx::{Pool, Postgres};

use auth::*;
use error::*;
use handlers::*;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod error;
mod handlers;
mod model;

pub const MEDIA_BUCKET: &str = "tapsters-media";
pub(crate) const BAR_TAG: &str = "bar";
pub(crate) const INGREDIENT_TAG: &str = "ingredient";
pub(crate) const MEDIA_TAG: &str = "media";
pub(crate) const MISC_TAG: &str = "misc";
pub(crate) const USER_TAG: &str = "user";

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

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Tapster API",
        description = "The REST CRUD API for Tapster",
        license(name = "AGPL-3.0", url = "https://github.com/ian-shakespeare/tapster/blob/main/LICENSE")
    ),
    servers(
        (url = "http://localhost:8000", description = "Local server")
    ),
    security(
        ("http" = [])
    ),
    tags(
        (name = BAR_TAG, description = "Bar API endpoints"),
        (name = INGREDIENT_TAG, description = "Ingredient API endpoints"),
        (name = MEDIA_TAG, description = "Media API endpoints"),
        (name = MISC_TAG, description = "Miscellaneous API endpoints"),
        (name = USER_TAG, description = "User and auth API endpoints"),
    )
)]
pub(crate) struct ApiDoc;

pub fn router(app_state: AppState) -> Router {
    let (router, docs) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(healthcheck_handler))
        .routes(routes!(list_units_handler))
        .routes(routes!(create_media_handler, get_media_handler,))
        .routes(routes!(create_bar_handler, list_bars_handler,))
        .routes(routes!(get_bar_handler))
        .routes(routes!(register_user_handler))
        .routes(routes!(sign_in_handler))
        .routes(routes!(create_ingredient_handler, list_ingredients_handler))
        .routes(routes!(get_ingredient_handler))
        .routes(routes!(get_ingredient_ingredients_handler))
        .with_state(Arc::new(app_state))
        .split_for_parts();

    router.merge(SwaggerUi::new("/swagger-ui").url("/docs/openapi.json", docs))
}
