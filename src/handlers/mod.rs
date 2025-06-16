use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    model::{UnitModel, UserModel},
    AppState, Auth,
};

pub(crate) use bar::*;
pub(crate) use ingredient::*;
pub(crate) use media::*;

mod bar;
mod ingredient;
mod media;

pub(crate) async fn healthcheck_handler() -> impl IntoResponse {
    Json(json!({
        "status": "OK",
        "service": "tapster-api",
    }))
}

pub(crate) async fn list_units_handler(
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    Ok(Json(UnitModel::all(&data.db).await?))
}

pub(crate) async fn register_user_handler(
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let user = UserModel::create(&data.db).await?;
    let auth = Auth::create(&data.signing_key, user)?;

    Ok(Json(auth))
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct SignIn {
    id: Uuid, // TODO: don't just use a uuid to authenticate
}

pub(crate) async fn sign_in_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<SignIn>,
) -> crate::Result<impl IntoResponse> {
    let user = UserModel::retrieve(&data.db, body.id).await?;
    let auth = Auth::create(&data.signing_key, user)?;

    Ok(Json(auth))
}
