use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemLog {
    pub id: i32,
    pub level: String,
    pub module: String,
    pub message: String,
    pub user_id: Option<i32>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl SystemLog {
    pub async fn create(
        pool: &MySqlPool,
        level: &str,
        module: &str,
        message: &str,
        user_id: Option<i32>,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<Self, AppError> {
        let log = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO system_logs (level, module, message, user_id, ip, user_agent)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            level,
            module,
            message,
            user_id,
            ip,
            user_agent
        )
        .execute(pool)
        .await?;

        Ok(Self {
            id: log.last_insert_id() as i32,
            level: level.to_string(),
            module: module.to_string(),
            message: message.to_string(),
            user_id,
            ip: ip.map(String::from),
            user_agent: user_agent.map(String::from),
            created_at: Utc::now(),
        })
    }

    pub async fn list(
        pool: &MySqlPool,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Self>, i64), AppError> {
        let offset = (page - 1) * per_page;
        
        let logs = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM system_logs 
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM system_logs"
        )
        .fetch_one(pool)
        .await?;

        Ok((logs, total.unwrap_or(0)))
    }
} 