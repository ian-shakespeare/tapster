use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{model::UserModel, AppState, Auth};

#[utoipa::path(
    post,
    path = "/register",
    responses(
        (status = CREATED, description = "Success", body = Auth, content_type = "application/json")
    ),
    tag = crate::USER_TAG
)]
pub(crate) async fn register_user_handler(
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let user = UserModel::create(&data.db).await?;
    let auth = Auth::create(&data.signing_key, user)?;

    Ok((StatusCode::CREATED, Json(auth)))
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct SignIn {
    id: Uuid, // TODO: don't just use a uuid to authenticate
}

#[utoipa::path(
    post,
    path = "/sign-in",
    request_body(content = SignIn, content_type = "application/json"),
    responses(
        (status = OK, description = "Success", body = Auth, content_type = "application/json")
    ),
    tag = crate::USER_TAG
)]
pub(crate) async fn sign_in_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<SignIn>,
) -> crate::Result<impl IntoResponse> {
    let user = UserModel::retrieve(&data.db, body.id).await?;
    let auth = Auth::create(&data.signing_key, user)?;

    Ok(Json(auth))
}
