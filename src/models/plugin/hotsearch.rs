use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct HotSearch {
    pub id: i32,
    pub platform: String,
    pub title: String,
    pub url: String,
    pub hot_value: i64,
    pub rank: i32,
    pub created_at: DateTime<Utc>,
}

impl HotSearch {
    pub async fn create_batch(
        pool: &MySqlPool,
        platform: &str,
        items: Vec<HotSearchItem>,
    ) -> AppResult<()> {
        // 先删除该平台的旧数据
        sqlx::query!(
            "DELETE FROM plugin_hotsearch WHERE platform = ?",
            platform
        )
        .execute(pool)
        .await?;

        // 批量插入新数据
        for (rank, item) in items.into_iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO plugin_hotsearch (platform, title, url, hot_value, rank)
                VALUES (?, ?, ?, ?, ?)
                "#,
                platform,
                item.title,
                item.url,
                item.hot_value,
                rank + 1
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    pub async fn list_by_platform(
        pool: &MySqlPool,
        platform: &str,
        limit: Option<i32>,
    ) -> AppResult<Vec<Self>> {
        let limit = limit.unwrap_or(50);

        let items = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_hotsearch
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

    pub async fn list_all(
        pool: &MySqlPool,
        page: u32,
        per_page: u32,
    ) -> AppResult<(Vec<Self>, i64)> {
        let offset = (page - 1) * per_page;

        let items = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_hotsearch
            ORDER BY platform ASC, rank ASC
            LIMIT ? OFFSET ?
            "#,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM plugin_hotsearch"
        )
        .fetch_one(pool)
        .await?;

        Ok((items, total.unwrap_or(0)))
    }

    pub async fn cleanup_old_data(pool: &MySqlPool) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM plugin_hotsearch
            WHERE created_at < DATE_SUB(NOW(), INTERVAL 1 DAY)
            "#
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotSearchItem {
    pub title: String,
    pub url: String,
    pub hot_value: i64,
} 