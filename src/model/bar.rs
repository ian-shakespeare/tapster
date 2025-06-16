use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Postgres};
use uuid::Uuid;

use super::MediaModel;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub(crate) struct BarModel {
    pub id: Uuid,
    pub name: String,
    pub owner: Uuid,
    pub created_at: NaiveDateTime,
    pub thumbnail: Option<MediaModel>,
}

impl BarModel {
    pub async fn create<'a, E>(
        executor: E,
        owner: Uuid,
        bar: CreateBarSchema,
    ) -> crate::Result<Self>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"
            WITH new_bars AS (
                INSERT INTO bars (
                    name,
                    media_id,
                    user_id
                ) VALUES (
                    lower($1),
                    $2,
                    $3
                ) RETURNING *
            )
            SELECT
                b.bar_id AS id,
                b.name,
                b.user_id AS owner,
                b.created_at,
                CASE
                    WHEN b.media_id IS NULL THEN NULL
                    ELSE json_build_object(
                        'id', m.media_id,
                        'size', m.size,
                        'mime_type', m.mime_type,
                        'created_at', m.created_at
                    )
                END AS "thumbnail: MediaModel"
            FROM new_bars b
            LEFT JOIN media m USING (user_id, media_id)
            "#,
            bar.name,
            bar.thumbnail_id,
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
            Self,
            r#"
            SELECT
                b.bar_id AS id,
                b.name,
                b.user_id AS owner,
                b.created_at,
                CASE
                    WHEN b.media_id IS NULL THEN NULL
                    ELSE json_build_object(
                        'id', m.media_id,
                        'size', m.size,
                        'mime_type', m.mime_type,
                        'created_at', m.created_at
                    )
                END AS "thumbnail: MediaModel"
            FROM bars b
            LEFT JOIN media m USING (user_id, media_id)
            WHERE b.user_id = $1
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
            Self,
            r#"
            SELECT
                b.bar_id AS id,
                b.name,
                b.user_id AS owner,
                b.created_at,
                CASE
                    WHEN b.media_id IS NULL THEN NULL
                    ELSE json_build_object(
                        'id', m.media_id,
                        'size', m.size,
                        'mime_type', m.mime_type,
                        'created_at', m.created_at
                    )
                END AS "thumbnail: MediaModel"
            FROM bars b
            LEFT JOIN media m USING (user_id, media_id)
            WHERE b.user_id = $1
                AND b.bar_id = $2
            "#,
            owner,
            id,
        )
        .fetch_one(executor)
        .await
        .map_err(|e| e.into())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct CreateBarSchema {
    pub name: String,
    #[serde(rename = "thumbnailId")]
    pub thumbnail_id: Option<Uuid>,
}
