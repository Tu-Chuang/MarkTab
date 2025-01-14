use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub user_agent: String,
    pub ip: String,
    pub expired_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Token {
    pub async fn create(
        pool: &sqlx::MySqlPool,
        user_id: i32,
        token: &str,
        user_agent: &str,
        ip: &str,
        expired_at: DateTime<Utc>,
    ) -> Result<Self, sqlx::Error> {
        let token = sqlx::query_as!(
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

        Ok(Self {
            id: token.last_insert_id() as i32,
            user_id,
            token: token.to_string(),
            user_agent: user_agent.to_string(),
            ip: ip.to_string(),
            expired_at,
            created_at: Utc::now(),
        })
    }

    pub async fn find_valid(pool: &sqlx::MySqlPool, token: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM MARKTAB_tokens
            WHERE token = ?
            AND expired_at > NOW()
            AND status = 1
            "#,
            token
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn invalidate(pool: &sqlx::MySqlPool, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE MARKTAB_tokens SET status = 0 WHERE token = ?",
            token
        )
        .execute(pool)
        .await?;

        Ok(())
    }
} 