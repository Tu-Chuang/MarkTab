mod models;
mod routes;

use async_trait::async_trait;
use actix_web::web;
use sqlx::MySqlPool;
use crate::{error::AppError, plugins::{Plugin, PluginInfo}};

pub use models::TopSearchItem;
pub use routes::config as routes_config;

pub struct TopSearchPlugin;

#[async_trait]
impl Plugin for TopSearchPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "热搜".to_string(),
            name_en: "top_search".to_string(),
            version: "1.0.0".to_string(),
            description: "各大平台热搜聚合".to_string(),
            author: "MARKTAB Team".to_string(),
            settings: Some(serde_json::json!({
                "baidu_code": "",
                "weibo_code": "",
                "zhihu_code": "",
                "cache_ttl": 180
            })),
        }
    }

    async fn install(&self, pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS plugin_top_search_items (
                id INT PRIMARY KEY AUTO_INCREMENT,
                platform VARCHAR(50) NOT NULL,
                title VARCHAR(255) NOT NULL,
                url VARCHAR(1024) NOT NULL,
                hot_value BIGINT NOT NULL DEFAULT 0,
                rank INT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                INDEX idx_platform_rank (platform, rank)
            )
            "#
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn uninstall(&self, pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query!("DROP TABLE IF EXISTS plugin_top_search_items")
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