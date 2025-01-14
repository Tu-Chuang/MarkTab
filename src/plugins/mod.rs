use async_trait::async_trait;
use actix_web::web;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

use crate::error::AppError;
use crate::models::plugin::PluginStatus;

pub mod todo;
pub mod poetry;
pub mod weather;
pub mod hotsearch;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    
    async fn install(&self, pool: &MySqlPool) -> Result<(), AppError>;
    async fn uninstall(&self, pool: &MySqlPool) -> Result<(), AppError>;
    async fn enable(&self, pool: &MySqlPool) -> Result<(), AppError>;
    async fn disable(&self, pool: &MySqlPool) -> Result<(), AppError>;
    
    fn configure_routes(&self, cfg: &mut web::ServiceConfig);
}

#[derive(Debug, Clone)]
pub struct PluginRegistry {
    plugins: Arc<RwLock<HashMap<String, Box<dyn Plugin>>>>,
    enabled_plugins: Arc<RwLock<HashMap<String, bool>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            enabled_plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, plugin: Box<dyn Plugin>) {
        let name = plugin.info().name_en;
        let mut plugins = self.plugins.blocking_write();
        plugins.insert(name, plugin);
    }

    pub async fn enable_plugin(&self, pool: &MySqlPool, name: &str) -> Result<(), AppError> {
        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(name) {
            plugin.enable(pool).await?;
            PluginStatus::set_enabled(pool, name, true).await?;
            let mut enabled = self.enabled_plugins.write().await;
            enabled.insert(name.to_string(), true);
            Ok(())
        } else {
            Err(AppError::PluginNotFound)
        }
    }

    pub async fn disable_plugin(&self, pool: &MySqlPool, name: &str) -> Result<(), AppError> {
        let plugins = self.plugins.read().await;
        if let Some(plugin) = plugins.get(name) {
            plugin.disable(pool).await?;
            PluginStatus::set_enabled(pool, name, false).await?;
            let mut enabled = self.enabled_plugins.write().await;
            enabled.insert(name.to_string(), false);
            Ok(())
        } else {
            Err(AppError::PluginNotFound)
        }
    }

    pub async fn init_from_db(&self, pool: &MySqlPool) -> Result<(), AppError> {
        let enabled_plugins = PluginStatus::list_enabled(pool).await?;
        let mut enabled = self.enabled_plugins.write().await;
        for name in enabled_plugins {
            enabled.insert(name, true);
        }
        Ok(())
    }

    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        let plugins = self.plugins.blocking_read();
        plugins.values().map(|p| p.info()).collect()
    }

    pub fn is_plugin_enabled(&self, name: &str) -> bool {
        let enabled = self.enabled_plugins.blocking_read();
        enabled.get(name).copied().unwrap_or(false)
    }

    pub fn configure_routes(&self, cfg: &mut web::ServiceConfig) {
        let plugins = self.plugins.blocking_read();
        for plugin in plugins.values() {
            plugin.configure_routes(cfg);
        }
    }
} 