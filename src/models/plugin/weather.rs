use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WeatherCache {
    pub id: i32,
    pub location: String,
    pub data: String,
    pub created_at: DateTime<Utc>,
}

impl WeatherCache {
    pub async fn set(
        pool: &MySqlPool,
        location: &str,
        data: &str,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO plugin_weather_cache (location, data)
            VALUES (?, ?)
            ON DUPLICATE KEY UPDATE
            data = VALUES(data)
            "#,
            location,
            data
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get(
        pool: &MySqlPool,
        location: &str,
    ) -> AppResult<Option<String>> {
        let cache = sqlx::query_scalar!(
            r#"
            SELECT data FROM plugin_weather_cache
            WHERE location = ?
            AND created_at > DATE_SUB(NOW(), INTERVAL 1 HOUR)
            "#,
            location
        )
        .fetch_optional(pool)
        .await?;

        Ok(cache)
    }
} 