use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub nickname: String,
    pub password: String,
    pub is_admin: bool,
    pub status: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_email(pool: &sqlx::MySqlPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_users WHERE email = ?",
            email
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_id(pool: &sqlx::MySqlPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM MARKTAB_users WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &sqlx::MySqlPool, new_user: &NewUser) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as!(
            Self,
            "INSERT INTO MARKTAB_users (email, nickname, password) VALUES (?, ?, ?)",
            new_user.email,
            new_user.nickname,
            new_user.password
        )
        .execute(pool)
        .await?;

        Ok(Self {
            id: user.last_insert_id() as i32,
            email: new_user.email.clone(),
            nickname: new_user.nickname.clone(),
            password: new_user.password.clone(),
            is_admin: false,
            status: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn update_password(
        pool: &sqlx::MySqlPool,
        id: i32,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE MARKTAB_users SET password = ? WHERE id = ?",
            password,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub nickname: String,
} 