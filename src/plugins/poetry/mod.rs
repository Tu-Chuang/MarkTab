mod models;
mod routes;

use async_trait::async_trait;
use actix_web::web;
use sqlx::MySqlPool;
use crate::{error::AppError, plugins::{Plugin, PluginInfo}};

pub use models::{Poetry, PoetryFavorite};
pub use routes::config as routes_config;

pub struct PoetryPlugin;

#[async_trait]
impl Plugin for PoetryPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "MARKTAB诗词".to_string(),
            name_en: "poetry".to_string(),
            version: "1.0.0".to_string(),
            description: "古诗词数据库".to_string(),
            author: "MARKTAB Team".to_string(),
            settings: None,
        }
    }

    async fn install(&self, pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS plugin_poetry (
                id INT PRIMARY KEY AUTO_INCREMENT,
                title VARCHAR(255) NOT NULL,
                content TEXT NOT NULL,
                author VARCHAR(255) NOT NULL,
                dynasty VARCHAR(255) NOT NULL,
                category VARCHAR(255) NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS plugin_poetry_favorites (
                id INT PRIMARY KEY AUTO_INCREMENT,
                user_id INT NOT NULL,
                poetry_id INT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES MARKTAB_users(id) ON DELETE CASCADE,
                FOREIGN KEY (poetry_id) REFERENCES plugin_poetry(id) ON DELETE CASCADE,
                UNIQUE KEY `unique_favorite` (user_id, poetry_id)
            )
            "#
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn uninstall(&self, pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query!("DROP TABLE IF EXISTS plugin_poetry_favorites")
            .execute(pool)
            .await?;
        sqlx::query!("DROP TABLE IF EXISTS plugin_poetry")
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