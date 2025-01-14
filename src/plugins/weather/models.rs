impl WeatherCache {
    pub async fn set(
        pool: &MySqlPool,
        location: &str,
        data: &str,
    ) -> Result<(), AppError> {
        // 删除旧缓存
        sqlx::query!(
            "DELETE FROM plugin_weather_cache WHERE location = ?",
            location
        )
        .execute(pool)
        .await?;

        // 插入新缓存
        sqlx::query!(
            r#"
            INSERT INTO plugin_weather_cache (location, data)
            VALUES (?, ?)
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
    ) -> Result<Option<String>, AppError> {
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

    pub async fn cleanup(pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM plugin_weather_cache
            WHERE created_at <= DATE_SUB(NOW(), INTERVAL 1 DAY)
            "#
        )
        .execute(pool)
        .await?;

        Ok(())
    }
} 