use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TokenRecord {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub user_agent: String,
    pub ip: String,
    pub expired_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl TokenRecord {
    pub async fn create(
        pool: &MySqlPool,
        user_id: i32,
        token: &str,
        user_agent: &str,
        ip: &str,
        expired_at: DateTime<Utc>,
    ) -> AppResult<Self> {
        let record = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO MARKTAB_tokens (user_id, token, user_agent, ip, expired_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
            user_id,
            token,
            user_agent,
            ip,
            expired_at
        )
        .execute(pool)
        .await?;

        Ok(Self::find_by_id(pool, record.last_insert_id() as i32).await?.unwrap())
    }

    pub async fn find_by_id(pool: &MySqlPool, id: i32) -> AppResult<Option<Self>> {
        let record = sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_tokens WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    pub async fn find_by_token(pool: &MySqlPool, token: &str) -> AppResult<Option<Self>> {
        let record = sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_tokens WHERE token = ? AND expired_at > NOW()",
            token
        )
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    pub async fn invalidate(pool: &MySqlPool, token: &str) -> AppResult<()> {
        sqlx::query!(
            "UPDATE MARKTAB_tokens SET expired_at = NOW() WHERE token = ?",
            token
        )
        .execute(pool)
        .await?;

        Ok(())
    }
} 