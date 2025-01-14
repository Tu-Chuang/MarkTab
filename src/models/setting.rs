use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Setting {
    pub id: i32,
    pub key: String,
    pub value: String,
}

impl Setting {
    pub async fn get(pool: &sqlx::MySqlPool, key: &str) -> Result<Option<Self>, sqlx::Error> {
        let setting = sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_settings WHERE `key` = ?",
            key
        )
        .fetch_optional(pool)
        .await?;

        Ok(setting)
    }

    pub async fn set(
        pool: &sqlx::MySqlPool,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO MARKTAB_settings (`key`, value)
            VALUES (?, ?)
            ON DUPLICATE KEY UPDATE
            value = VALUES(value)
            "#,
            key,
            value.to_string()
        )
        .execute(pool)
        .await?;

        Ok(())
    }
} 