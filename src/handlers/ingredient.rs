use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    model::{CreateIngredientSchema, IngredientModel},
    AppState, Auth,
};

pub(crate) async fn create_ingredient_handler(
    header: HeaderMap,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateIngredientSchema>,
) -> crate::Result<impl IntoResponse> {
    let owner = Auth::decode_header(&data.signing_key, header)?;

    Ok(Json(IngredientModel::create(&data.db, owner, body).await?))
}

pub(crate) async fn list_ingredients_handler(
    header: HeaderMap,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner = Auth::decode_header(&data.signing_key, header)?;

    Ok(Json(IngredientModel::all(&data.db, owner).await?))
}

pub(crate) async fn get_ingredient_handler(
    header: HeaderMap,
    Path(ingredient_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner = Auth::decode_header(&data.signing_key, header)?;

    Ok(Json(
        IngredientModel::retrieve(&data.db, owner, ingredient_id).await?,
    ))
}

pub(crate) async fn get_ingredient_ingredients_handler(
    header: HeaderMap,
    Path(ingredient_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner = Auth::decode_header(&data.signing_key, header)?;
    let ingredient = IngredientModel::retrieve(&data.db, owner, ingredient_id).await?;

    Ok(Json(ingredient.ingredients(&data.db).await?))
}
