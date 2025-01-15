use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub nickname: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_id(pool: &MySqlPool, id: i32) -> AppResult<Option<Self>> {
        let user = sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_users WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &MySqlPool, email: &str) -> AppResult<Option<Self>> {
        let user = sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_users WHERE email = ?",
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn create(
        pool: &MySqlPool,
        email: &str,
        password: &str,
        nickname: &str,
    ) -> AppResult<Self> {
        let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        let user = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO MARKTAB_users (email, password, nickname)
            VALUES (?, ?, ?)
            "#,
            email,
            hashed,
            nickname
        )
        .execute(pool)
        .await?;

        Ok(Self::find_by_id(pool, user.last_insert_id() as i32).await?.unwrap())
    }
} 