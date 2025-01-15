pub mod hotsearch;
pub mod poetry;
pub mod todo;
pub mod weather;

use async_trait::async_trait;
use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use crate::error::AppResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub name_en: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub settings: Option<serde_json::Value>,
}

#[async_trait]
pub trait Plugin: Send + Sync {
    fn info(&self) -> PluginInfo;
    
    async fn install(&self, pool: &MySqlPool) -> AppResult<()>;
    
    async fn uninstall(&self, pool: &MySqlPool) -> AppResult<()>;
    
    async fn enable(&self, pool: &MySqlPool) -> AppResult<()>;
    
    async fn disable(&self, pool: &MySqlPool) -> AppResult<()>;
    
    fn configure_routes(&self, _cfg: &mut web::ServiceConfig) {}
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: vec![
                Box::new(weather::WeatherPlugin),
                Box::new(poetry::PoetryPlugin),
                Box::new(todo::TodoPlugin),
                Box::new(hotsearch::HotSearchPlugin),
            ],
        }
    }

    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins.iter().map(|p| p.info()).collect()
    }

    pub async fn enable_plugin(&self, pool: &MySqlPool, name: &str) -> AppResult<()> {
        if let Some(plugin) = self.find_plugin(name) {
            plugin.enable(pool).await?;
        }
        Ok(())
    }

    pub async fn disable_plugin(&self, pool: &MySqlPool, name: &str) -> AppResult<()> {
        if let Some(plugin) = self.find_plugin(name) {
            plugin.disable(pool).await?;
        }
        Ok(())
    }

    fn find_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.iter().find(|p| p.info().name_en == name)
    }
} 