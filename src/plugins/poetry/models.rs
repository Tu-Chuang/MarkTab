use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Poetry {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: String,
    pub dynasty: Option<String>,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Poetry {
    pub async fn create(pool: &MySqlPool, poetry: &NewPoetry) -> Result<Self, AppError> {
        let poetry = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO plugin_poetry (title, content, author, dynasty, category)
            VALUES (?, ?, ?, ?, ?)
            "#,
            poetry.title,
            poetry.content,
            poetry.author,
            poetry.dynasty,
            poetry.category
        )
        .execute(pool)
        .await?;

        Ok(Self {
            id: poetry.last_insert_id() as i32,
            title: poetry.title.clone(),
            content: poetry.content.clone(),
            author: poetry.author.clone(),
            dynasty: poetry.dynasty.clone(),
            category: poetry.category.clone(),
            created_at: Utc::now(),
        })
    }

    pub async fn random(pool: &MySqlPool) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_poetry
            ORDER BY RAND()
            LIMIT 1
            "#
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn search(
        pool: &MySqlPool,
        keyword: &str,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Self>, i64), AppError> {
        let offset = (page - 1) * per_page;
        let keyword = format!("%{}%", keyword);

        let poems = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_poetry
            WHERE title LIKE ? OR content LIKE ? OR author LIKE ?
            ORDER BY created_at DESC
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PoetryFavorite {
    pub id: i32,
    pub user_id: i32,
    pub poetry_id: i32,
    pub created_at: DateTime<Utc>,
}

impl PoetryFavorite {
    pub async fn create(
        pool: &MySqlPool,
        user_id: i32,
        poetry_id: i32,
    ) -> Result<Self, AppError> {
        let favorite = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO plugin_poetry_favorites (user_id, poetry_id)
            VALUES (?, ?)
            "#,
            user_id,
            poetry_id
        )
        .execute(pool)
        .await?;

        Ok(Self {
            id: favorite.last_insert_id() as i32,
            user_id,
            poetry_id,
            created_at: Utc::now(),
        })
    }

    pub async fn delete(
        pool: &MySqlPool,
        user_id: i32,
        poetry_id: i32,
    ) -> Result<(), AppError> {
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
    ) -> Result<(Vec<Poetry>, i64), AppError> {
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

    pub async fn is_favorite(
        pool: &MySqlPool,
        user_id: i32,
        poetry_id: i32,
    ) -> Result<bool, AppError> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM plugin_poetry_favorites
            WHERE user_id = ? AND poetry_id = ?
            "#,
            user_id,
            poetry_id
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0) > 0)
    }
} 