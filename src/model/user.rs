use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Postgres};
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub(crate) struct UserModel {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
}

impl UserModel {
    pub async fn create<'a, E>(executor: E) -> crate::Result<Self>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"
            WITH new_users AS (
                INSERT INTO users DEFAULT VALUES RETURNING *
            )
            SELECT
                user_id AS id,
                created_at
            FROM new_users
            "#,
        )
        .fetch_one(executor)
        .await
        .map_err(|e| e.into())
    }

    pub async fn retrieve<'a, E>(executor: E, id: Uuid) -> crate::Result<Self>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"
            SELECT
                user_id AS id,
                created_at
            FROM users
            WHERE user_id = $1
            "#,
            id
        )
        .fetch_one(executor)
        .await
        .map_err(|e| e.into())
    }
}
