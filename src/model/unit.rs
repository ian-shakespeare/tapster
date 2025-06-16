use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Postgres};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
pub(crate) struct UnitModel {
    id: Uuid,
    name: String,
    abbreviation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

impl UnitModel {
    pub async fn all<'a, E>(executor: E) -> crate::Result<Vec<Self>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"
            SELECT
                u.unit_id AS id,
                u.name,
                u.abbreviation,
                us.name AS system
            FROM units u
            LEFT JOIN unit_systems us USING (unit_system_id)
            "#,
        )
        .fetch_all(executor)
        .await
        .map_err(|e| e.into())
    }
}
