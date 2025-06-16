use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

use crate::{model::UnitModel, AppState};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct HealthCheck {
    status: String,
    service: String,
}

#[utoipa::path(
    get,
    path = "/healthcheck",
    responses(
        (status = OK, description = "Success", body = HealthCheck, content_type = "application/json")
    ),
    tag = crate::MISC_TAG
)]
pub(crate) async fn healthcheck_handler() -> impl IntoResponse {
    let healthcheck = HealthCheck {
        status: "OK".to_string(),
        service: "tapster-api".to_string(),
    };

    Json(healthcheck)
}

#[utoipa::path(
    get,
    path = "/units",
    responses(
        (status = OK, description = "Success", body = Vec<UnitModel>, content_type = "application/json")
    ),
    tag = crate::MISC_TAG
)]
pub(crate) async fn list_units_handler(
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    Ok(Json(UnitModel::all(&data.db).await?))
}
