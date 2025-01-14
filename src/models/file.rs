use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub user_id: Option<i32>,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    pub hash: String,
    pub created_at: DateTime<Utc>,
}

impl File {
    pub async fn create(
        pool: &MySqlPool,
        user_id: i32,
        filename: &str,
        mime_type: &str,
        size: i64,
        hash: &str,
        path: &str,
    ) -> Result<Self, AppError> {
        let file = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO MARKTAB_files (user_id, filename, mime_type, size, hash, path)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            user_id,
            filename,
            mime_type,
            size,
            hash,
            path
        )
        .execute(pool)
        .await?;

        Ok(Self {
            id: file.last_insert_id() as i32,
            user_id,
            path: path.to_string(),
            mime_type: mime_type.to_string(),
            size,
            hash: hash.to_string(),
            created_at: Utc::now(),
        })
    }

    pub async fn find_by_hash(pool: &MySqlPool, hash: &str) -> Result<Option<Self>, AppError> {
        let file = sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_files WHERE hash = ?",
            hash
        )
        .fetch_optional(pool)
        .await?;

        Ok(file)
    }

    pub async fn find_by_id(pool: &MySqlPool, id: i32) -> Result<Option<Self>, AppError> {
        let file = sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_files WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(file)
    }

    pub async fn list_by_user(
        pool: &MySqlPool,
        user_id: i32,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Self>, i64), AppError> {
        let offset = (page - 1) * per_page;
        
        let files = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM MARKTAB_files
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM MARKTAB_files WHERE user_id = ?",
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok((files, total.unwrap_or(0)))
    }
} 