use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use chrono::{DateTime, Utc};
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TodoFolder {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub folder_id: i32,
    pub content: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TodoFolder {
    pub async fn create(
        pool: &MySqlPool,
        user_id: i32,
        name: &str,
    ) -> AppResult<Self> {
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

        Ok(Self::find_by_id(pool, folder.last_insert_id() as i32).await?.unwrap())
    }

    pub async fn find_by_id(pool: &MySqlPool, id: i32) -> AppResult<Option<Self>> {
        let folder = sqlx::query_as!(
            Self,
            "SELECT * FROM plugin_todo_folders WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(folder)
    }

    pub async fn list_by_user(pool: &MySqlPool, user_id: i32) -> AppResult<Vec<Self>> {
        let folders = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_todo_folders
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(folders)
    }
}

impl Todo {
    pub async fn create(
        pool: &MySqlPool,
        user_id: i32,
        folder_id: i32,
        content: &str,
    ) -> AppResult<Self> {
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

        Ok(Self::find_by_id(pool, todo.last_insert_id() as i32).await?.unwrap())
    }

    pub async fn find_by_id(pool: &MySqlPool, id: i32) -> AppResult<Option<Self>> {
        let todo = sqlx::query_as!(
            Self,
            "SELECT * FROM plugin_todos WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(todo)
    }

    pub async fn list_by_folder(
        pool: &MySqlPool,
        folder_id: i32,
    ) -> AppResult<Vec<Self>> {
        let todos = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_todos
            WHERE folder_id = ?
            ORDER BY created_at DESC
            "#,
            folder_id
        )
        .fetch_all(pool)
        .await?;

        Ok(todos)
    }

    pub async fn update_status(
        pool: &MySqlPool,
        id: i32,
        completed: bool,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
            UPDATE plugin_todos
            SET completed = ?
            WHERE id = ?
            "#,
            completed,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
} 