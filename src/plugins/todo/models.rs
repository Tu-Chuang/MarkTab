use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub folder_id: i32,
    pub content: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Todo {
    pub async fn create(
        pool: &MySqlPool,
        user_id: i32,
        folder_id: i32,
        content: &str,
    ) -> Result<Self, AppError> {
        let todo = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO plugin_todos (user_id, folder_id, content)
            VALUES (?, ?, ?)
            "#,
            user_id,
            folder_id,
            content
        )
        .execute(pool)
        .await?;

        Ok(Self {
            id: todo.last_insert_id() as i32,
            user_id,
            folder_id,
            content: content.to_string(),
            completed: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn list_by_folder(
        pool: &MySqlPool,
        user_id: i32,
        folder_id: i32,
    ) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_todos
            WHERE user_id = ? AND folder_id = ?
            ORDER BY created_at DESC
            "#,
            user_id,
            folder_id
        )
        .fetch_all(pool)
        .await
    }
}

impl Folder {
    pub async fn create(
        pool: &MySqlPool,
        user_id: i32,
        name: &str,
    ) -> Result<Self, AppError> {
        let folder = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO plugin_todo_folders (user_id, name)
            VALUES (?, ?)
            "#,
            user_id,
            name
        )
        .execute(pool)
        .await?;

        Ok(Self {
            id: folder.last_insert_id() as i32,
            user_id,
            name: name.to_string(),
            created_at: Utc::now(),
        })
    }

    pub async fn list_by_user(
        pool: &MySqlPool,
        user_id: i32,
    ) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_todo_folders
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
    }
} 