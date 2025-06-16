use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Executor, Postgres, Type};
use utoipa::ToSchema;
use uuid::Uuid;

use super::MediaModel;

#[derive(Debug, FromRow, Type, Deserialize, Serialize, ToSchema)]
pub(crate) struct IngredientModel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub owner: Uuid,
    pub created_at: NaiveDateTime,
    #[sqlx(json(nullable))]
    pub thumbnail: Option<MediaModel>,
}

impl IngredientModel {
    pub async fn create<'a, E>(
        executor: E,
        owner: Uuid,
        ingredient: CreateIngredientSchema,
    ) -> crate::Result<Self>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"
            WITH new_ingredients AS (
                INSERT INTO ingredients (
                    name,
                    description,
                    media_id,
                    user_id
                ) VALUES (
                    lower($1),
                    $2,
                    $3,
                    $4
                ) RETURNING *
            )
            SELECT
                i.ingredient_id AS id,
                i.name,
                i.description,
                i.user_id AS owner,
                i.created_at,
                CASE
                    WHEN i.media_id IS NULL THEN NULL
                    ELSE json_build_object(
                        'id', m.media_id,
                        'size', m.size,
                        'mime_type', m.mime_type,
                        'created_at', m.created_at
                    )
                END AS "thumbnail: MediaModel"
            FROM new_ingredients i
            LEFT JOIN media m USING (media_id)
            "#,
            ingredient.name,
            ingredient.description,
            ingredient.thumbnail_id,
            owner
        )
        .fetch_one(executor)
        .await
        .map_err(|e| e.into())
    }

    pub async fn all<'a, E>(executor: E, owner: Uuid) -> crate::Result<Vec<Self>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            IngredientModel,
            r#"
            SELECT
                i.ingredient_id AS id,
                i.name,
                i.description,
                i.user_id AS owner,
                i.created_at,
                CASE
                    WHEN i.media_id IS NULL THEN NULL
                    ELSE json_build_object(
                        'id', m.media_id,
                        'size', m.size,
                        'mime_type', m.mime_type,
                        'created_at', m.created_at
                    )
                END AS "thumbnail: MediaModel"
            FROM ingredients i
            LEFT JOIN media m USING (user_id, media_id)
            WHERE i.user_id = $1
            "#,
            owner
        )
        .fetch_all(executor)
        .await
        .map_err(|e| e.into())
    }

    pub async fn retrieve<'a, E>(executor: E, owner: Uuid, id: Uuid) -> crate::Result<Self>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            IngredientModel,
            r#"
            SELECT
                i.ingredient_id AS id,
                i.name,
                i.description,
                i.user_id AS owner,
                i.created_at,
                CASE
                    WHEN i.media_id IS NULL THEN NULL
                    ELSE json_build_object(
                        'id', m.media_id,
                        'size', m.size,
                        'mime_type', m.mime_type,
                        'created_at', m.created_at
                    )
                END AS "thumbnail: MediaModel"
            FROM ingredients i
            LEFT JOIN media m USING (user_id, media_id)
            WHERE i.user_id = $1
                AND i.ingredient_id = $2
            "#,
            owner,
            id,
        )
        .fetch_one(executor)
        .await
        .map_err(|e| e.into())
    }

    pub async fn ingredients<'a, E>(self, executor: E) -> crate::Result<Vec<SubIngredientModel>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            SubIngredientModel,
            r#"
            SELECT
                ii.ingredient_ingredient_id AS id,
                ii.parts,
                json_build_object(
                    'id', i.ingredient_id,
                    'name', i.name,
                    'description', i.description,
                    'owner', i.user_id,
                    'created_at', i.created_at,
                    'thumbnail', CASE
                        WHEN i.media_id IS NULL THEN NULL
                        ELSE json_build_object(
                            'id', m.media_id,
                            'size', m.size,
                            'mime_type', m.mime_type,
                            'created_at', m.created_at
                        )
                    END
                ) AS "ingredient: IngredientModel"
            FROM ingredient_ingredients ii
            JOIN ingredients i USING (ingredient_id)
            JOIN media m USING (media_id)
            WHERE i.user_id = $1
                AND ii.compound_ingredient_id = $2
            "#,
            self.owner,
            self.id,
        )
        .fetch_all(executor)
        .await
        .map_err(|e| e.into())
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub(crate) struct CreateIngredientSchema {
    pub name: String,
    pub description: String,
    #[serde(rename = "thumbnailId")]
    pub thumbnail_id: Option<Uuid>,
}

#[derive(Debug, FromRow, Type, Deserialize, Serialize, ToSchema)]
pub(crate) struct SubIngredientModel {
    pub id: Uuid,
    pub parts: i16,
    #[sqlx(json)]
    pub ingredient: Option<IngredientModel>,
}
