use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Executor, Postgres, Type};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromRow, Type, Deserialize, Serialize, ToSchema)]
pub(crate) struct MediaModel {
    pub id: Uuid,
    pub size: i64,
    pub content_type: String,
    pub owner: Uuid,
    pub created_at: NaiveDateTime,
}

impl MediaModel {
    pub async fn create<'a, E>(executor: E, owner: Uuid) -> crate::Result<Self>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"
            WITH new_media AS (
                INSERT INTO media (user_id) VALUES ($1) RETURNING *
            )
            SELECT
                media_id AS id,
                size,
                mime_type AS content_type,
                user_id AS owner,
                created_at
            FROM new_media
            "#,
            owner
        )
        .fetch_one(executor)
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
                media_id AS id,
                size,
                mime_type AS content_type,
                user_id AS owner,
                created_at
            FROM media
            WHERE user_id = $1
                AND media_id = $2
            "#,
            owner,
            id,
        )
        .fetch_one(executor)
        .await
        .map_err(|e| e.into())
    }

    pub async fn update<'a, E>(
        self,
        executor: E,
        owner: Uuid,
        media: UpdateMediaSchema,
    ) -> crate::Result<Self>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"
            WITH updated_media AS (
                UPDATE media m
                SET
                    size = COALESCE($1, m.size),
                    mime_type = COALESCE($2, m.mime_type)
                WHERE m.user_id = $3
                    AND m.media_id = $4
                RETURNING *
            )
            SELECT
                media_id AS id,
                size,
                mime_type AS content_type,
                user_id AS owner,
                created_at
            FROM updated_media
            "#,
            media.size,
            media.content_type,
            owner,
            self.id,
        )
        .fetch_one(executor)
        .await
        .map_err(|e| e.into())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct UpdateMediaSchema {
    pub size: Option<i64>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
}
