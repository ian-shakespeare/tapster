use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    model::{CreateIngredientSchema, IngredientModel, SubIngredientModel},
    AppState, Auth,
};

#[utoipa::path(
    post,
    path = "/ingredients",
    request_body(content = CreateIngredientSchema, content_type = "application/json"),
    responses(
        (status = CREATED, description = "Success", body = IngredientModel, content_type = "application/json")
    ),
    security(
        ("http" = [])
    ),
    tag = crate::INGREDIENT_TAG
)]
pub(crate) async fn create_ingredient_handler(
    header: HeaderMap,
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateIngredientSchema>,
) -> crate::Result<impl IntoResponse> {
    let owner = Auth::decode_header(&data.signing_key, header)?;

    Ok((
        StatusCode::CREATED,
        Json(IngredientModel::create(&data.db, owner, body).await?),
    ))
}

#[utoipa::path(
    get,
    path = "/ingredients",
    responses(
        (status = OK, description = "Success", body = Vec<IngredientModel>, content_type = "application/json")
    ),
    security(
        ("http" = [])
    ),
    tag = crate::INGREDIENT_TAG
)]
pub(crate) async fn list_ingredients_handler(
    header: HeaderMap,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner = Auth::decode_header(&data.signing_key, header)?;

    Ok(Json(IngredientModel::all(&data.db, owner).await?))
}

#[utoipa::path(
    get,
    path = "/ingredients/{ingredient_id}",
    params(
        ("ingredient_id" = Uuid, Path, description = "ID of the ingredient to retrieve")
    ),
    responses(
        (status = OK, description = "Success", body = IngredientModel, content_type = "application/json")
    ),
    security(
        ("http" = [])
    ),
    tag = crate::INGREDIENT_TAG
)]
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

#[utoipa::path(
    get,
    path = "/ingredients/{ingredient_id}/ingredients",
    params(
        ("ingredient_id" = Uuid, Path, description = "ID of the ingredient to retrieve")
    ),
    responses(
        (status = OK, description = "Success", body = Vec<SubIngredientModel>, content_type = "application/json")
    ),
    security(
        ("http" = [])
    ),
    tag = crate::INGREDIENT_TAG
)]
pub(crate) async fn get_ingredient_ingredients_handler(
    header: HeaderMap,
    Path(ingredient_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> crate::Result<impl IntoResponse> {
    let owner = Auth::decode_header(&data.signing_key, header)?;
    let ingredient = IngredientModel::retrieve(&data.db, owner, ingredient_id).await?;

    Ok(Json(ingredient.ingredients(&data.db).await?))
}
