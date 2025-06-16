use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    model::{BarModel, CreateBarSchema},
    AppState, Auth,
};

pub(crate) async fn create_bar_handler(
    header: HeaderMap,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateBarSchema>,
) -> crate::Result<impl IntoResponse> {
    let owner_id = Auth::decode_header(&data.signing_key, header)?;

    Ok(Json(BarModel::create(&data.db, owner_id, body).await?))
}

pub(crate) async fn list_bars_handler(
    header: HeaderMap,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner_id = Auth::decode_header(&data.signing_key, header)?;

    Ok(Json(BarModel::all(&data.db, owner_id).await?))
}

pub(crate) async fn get_bar_handler(
    header: HeaderMap,
    Path(bar_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner_id = Auth::decode_header(&data.signing_key, header)?;

    Ok(Json(BarModel::retrieve(&data.db, owner_id, bar_id).await?))
}
