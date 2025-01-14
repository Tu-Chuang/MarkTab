use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TopSearchItem {
    pub id: i32,
    pub platform: String,
    pub title: String,
    pub url: String,
    pub hot_value: i64,
    pub rank: i32,
    pub created_at: DateTime<Utc>,
}

impl TopSearchItem {
    pub async fn list_by_platform(
        pool: &MySqlPool,
        platform: &str,
        limit: i32,
    ) -> Result<Vec<Self>, AppError> {
        let items = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_top_search_items
            WHERE platform = ?
            ORDER BY rank ASC
            LIMIT ?
            "#,
            platform,
            limit
        )
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    pub async fn batch_create(
        pool: &MySqlPool,
        platform: &str,
        items: Vec<TopSearchItem>,
    ) -> Result<(), AppError> {
        // 先删除该平台的旧数据
        sqlx::query!(
            "DELETE FROM plugin_top_search_items WHERE platform = ?",
            platform
        )
        .execute(pool)
        .await?;

        // 批量插入新数据
        for item in items {
            sqlx::query!(
                r#"
                INSERT INTO plugin_top_search_items 
                (platform, title, url, hot_value, rank)
                VALUES (?, ?, ?, ?, ?)
                "#,
                platform,
                item.title,
                item.url,
                item.hot_value,
                item.rank
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }
} 