mod models;
mod routes;

use async_trait::async_trait;
use serde_json::json;
use sqlx::MySqlPool;
use crate::{error::AppError, plugins::{Plugin, PluginInfo}};

pub use models::{Todo, Folder};
pub use routes::config as routes_config;

pub struct TodoPlugin;

#[async_trait]
impl Plugin for TodoPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "MARKTAB待办事项".to_string(),
            name_en: "todo".to_string(),
            version: "1.0.0".to_string(),
            description: "待办事项管理".to_string(),
            author: "MARKTAB Team".to_string(),
            settings: None,
        }
    }

    async fn install(&self, pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS plugin_todo_folders (
                id INT PRIMARY KEY AUTO_INCREMENT,
                user_id INT NOT NULL,
                name VARCHAR(255) NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES MARKTAB_users(id) ON DELETE CASCADE
            )
            "#
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS plugin_todos (
                id INT PRIMARY KEY AUTO_INCREMENT,
                user_id INT NOT NULL,
                folder_id INT NOT NULL,
                content TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES MARKTAB_users(id) ON DELETE CASCADE,
                FOREIGN KEY (folder_id) REFERENCES plugin_todo_folders(id) ON DELETE CASCADE
            )
            "#
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn uninstall(&self, pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query!("DROP TABLE IF EXISTS plugin_todos")
            .execute(pool)
            .await?;
        sqlx::query!("DROP TABLE IF EXISTS plugin_todo_folders")
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn enable(&self, _pool: &MySqlPool) -> Result<(), AppError> {
        Ok(())
    }

    async fn disable(&self, _pool: &MySqlPool) -> Result<(), AppError> {
        Ok(())
    }

    fn configure_routes(&self, cfg: &mut web::ServiceConfig) {
        routes_config(cfg);
    }
} 