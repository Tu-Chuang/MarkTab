use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Poetry {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: String,
    pub dynasty: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PoetryFavorite {
    pub id: i32,
    pub user_id: i32,
    pub poetry_id: i32,
    pub created_at: DateTime<Utc>,
}

impl Poetry {
    pub async fn random(pool: &MySqlPool) -> AppResult<Option<Self>> {
        let poetry = sqlx::query_as!(
            Self,
            "SELECT * FROM plugin_poetry ORDER BY RAND() LIMIT 1"
        )
        .fetch_optional(pool)
        .await?;

        Ok(poetry)
    }

    pub async fn search(
        pool: &MySqlPool,
        keyword: &str,
        page: u32,
        per_page: u32,
    ) -> AppResult<(Vec<Self>, i64)> {
        let offset = (page - 1) * per_page;
        let keyword = format!("%{}%", keyword);

        let poems = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_poetry
            WHERE title LIKE ? OR content LIKE ? OR author LIKE ?
            ORDER BY id DESC
            LIMIT ? OFFSET ?
            "#,
            keyword,
            keyword,
            keyword,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM plugin_poetry
            WHERE title LIKE ? OR content LIKE ? OR author LIKE ?
            "#,
            keyword,
            keyword,
            keyword
        )
        .fetch_one(pool)
        .await?;

        Ok((poems, total.unwrap_or(0)))
    }
}

impl PoetryFavorite {
    pub async fn add(
        pool: &MySqlPool,
        user_id: i32,
        poetry_id: i32,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT IGNORE INTO plugin_poetry_favorites (user_id, poetry_id)
            VALUES (?, ?)
            "#,
            user_id,
            poetry_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn remove(
        pool: &MySqlPool,
        user_id: i32,
        poetry_id: i32,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM plugin_poetry_favorites
            WHERE user_id = ? AND poetry_id = ?
            "#,
            user_id,
            poetry_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn list_by_user(
        pool: &MySqlPool,
        user_id: i32,
        page: u32,
        per_page: u32,
    ) -> AppResult<(Vec<Poetry>, i64)> {
        let offset = (page - 1) * per_page;

        let poems = sqlx::query_as!(
            Poetry,
            r#"
            SELECT p.* FROM plugin_poetry p
            INNER JOIN plugin_poetry_favorites f ON f.poetry_id = p.id
            WHERE f.user_id = ?
            ORDER BY f.created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM plugin_poetry_favorites
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok((poems, total.unwrap_or(0)))
    }
} 