use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    model::{BarModel, CreateBarSchema},
    AppState, Auth,
};

#[utoipa::path(
    post,
    path = "/bars",
    request_body(content = CreateBarSchema, content_type = "application/json"),
    responses(
        (status = CREATED, description = "Success", body = BarModel, content_type = "application/json")
    ),
    security(
        ("http" = [])
    ),
    tag = crate::BAR_TAG
)]
pub(crate) async fn create_bar_handler(
    header: HeaderMap,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateBarSchema>,
) -> crate::Result<impl IntoResponse> {
    let owner_id = Auth::decode_header(&data.signing_key, header)?;

    Ok((
        StatusCode::CREATED,
        Json(BarModel::create(&data.db, owner_id, body).await?),
    ))
}

#[utoipa::path(
    get,
    path = "/bars",
    responses(
        (status = OK, description = "Success", body = Vec<BarModel>, content_type = "application/json")
    ),
    security(
        ("http" = [])
    ),
    tag = crate::BAR_TAG
)]
pub(crate) async fn list_bars_handler(
    header: HeaderMap,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner_id = Auth::decode_header(&data.signing_key, header)?;

    Ok(Json(BarModel::all(&data.db, owner_id).await?))
}

#[utoipa::path(
    get,
    path = "/bars/{bar_id}",
    params(
        ("bar_id" = Uuid, Path, description = "ID of the bar to retrieve")
    ),
    responses(
        (status = OK, description = "Success", body = BarModel, content_type = "application/json")
    ),
    security(
        ("http" = [])
    ),
    tag = crate::BAR_TAG
)]
pub(crate) async fn get_bar_handler(
    header: HeaderMap,
    Path(bar_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner_id = Auth::decode_header(&data.signing_key, header)?;

    Ok(Json(BarModel::retrieve(&data.db, owner_id, bar_id).await?))
}
