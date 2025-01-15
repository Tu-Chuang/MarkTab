use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Setting {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Setting {
    pub async fn get(pool: &MySqlPool, key: &str) -> AppResult<Option<Self>> {
        let setting = sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_settings WHERE `key` = ?",
            key
        )
        .fetch_optional(pool)
        .await?;

        Ok(setting)
    }

    pub async fn set(pool: &MySqlPool, key: &str, value: &str) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO MARKTAB_settings (`key`, value)
            VALUES (?, ?)
            ON DUPLICATE KEY UPDATE
            value = VALUES(value)
            "#,
            key,
            value
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(pool: &MySqlPool, key: &str) -> AppResult<()> {
        sqlx::query!(
            "DELETE FROM MARKTAB_settings WHERE `key` = ?",
            key
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn list(
        pool: &MySqlPool,
        page: u32,
        per_page: u32,
    ) -> AppResult<(Vec<Self>, i64)> {
        let offset = (page - 1) * per_page;

        let settings = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM MARKTAB_settings
            ORDER BY `key`
            LIMIT ? OFFSET ?
            "#,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM MARKTAB_settings"
        )
        .fetch_one(pool)
        .await?;

        Ok((settings, total.unwrap_or(0)))
    }
} 