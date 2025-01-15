use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PluginStatus {
    pub id: i32,
    pub name: String,
    pub enabled: bool,
    pub settings: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PluginStatus {
    pub async fn find_by_name(pool: &MySqlPool, name: &str) -> AppResult<Option<Self>> {
        let status = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM MARKTAB_plugin_status
            WHERE name = ?
            "#,
            name
        )
        .fetch_optional(pool)
        .await?;

        Ok(status)
    }

    pub async fn set_enabled(
        pool: &MySqlPool,
        name: &str,
        enabled: bool,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO MARKTAB_plugin_status (name, enabled)
            VALUES (?, ?)
            ON DUPLICATE KEY UPDATE
            enabled = VALUES(enabled)
            "#,
            name,
            enabled
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_settings(
        pool: &MySqlPool,
        name: &str,
        settings: &serde_json::Value,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE MARKTAB_plugin_status
            SET settings = ?
            WHERE name = ?
            "#,
            settings.to_string(),
            name
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn list_enabled(pool: &MySqlPool) -> AppResult<Vec<String>> {
        let plugins = sqlx::query!(
            r#"
            SELECT name FROM MARKTAB_plugin_status
            WHERE enabled = TRUE
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(plugins.into_iter().map(|p| p.name).collect())
    }
} 