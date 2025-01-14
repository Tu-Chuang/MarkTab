use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use crate::{error::AppError, plugins::{Plugin, PluginInfo}};

pub struct WeatherPlugin;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherConfig {
    pub api_key: String,
    pub gateway: String,
}

#[async_trait]
impl Plugin for WeatherPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "MARKTAB天气".to_string(),
            name_en: "weather".to_string(),
            version: "1.0.0".to_string(),
            description: "天气预报服务".to_string(),
            author: "MARKTAB Team".to_string(),
            settings: Some(json!({
                "api_key": "your_api_key_here"
            })),
        }
    }

    async fn install(&self, pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS plugin_weather_cache (
                id INT PRIMARY KEY AUTO_INCREMENT,
                location VARCHAR(255) NOT NULL,
                data TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn uninstall(&self, pool: &MySqlPool) -> Result<(), AppError> {
        sqlx::query!("DROP TABLE IF EXISTS plugin_weather_cache")
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
} 