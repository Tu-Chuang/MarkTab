use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{
    error::AppError,
    plugins::{Plugin, PluginInfo},
    models::setting::Setting,
};

#[derive(Deserialize)]
pub struct PluginAction {
    name: String,
    action: String, // install, uninstall, enable, disable
}

#[derive(Deserialize)]
pub struct PluginConfig {
    name: String,
    settings: serde_json::Value,
}

pub async fn list_plugins() -> Result<HttpResponse, AppError> {
    let plugins = vec![
        "weather",
        "poetry",
        "todo",
        "top_search"
    ];

    let mut plugin_infos = Vec::new();
    for name in plugins {
        if let Some(plugin) = get_plugin(name) {
            plugin_infos.push(plugin.info());
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": plugin_infos
    })))
}

pub async fn handle_plugin(
    pool: web::Data<sqlx::MySqlPool>,
    action: web::Json<PluginAction>,
) -> Result<HttpResponse, AppError> {
    let plugin = get_plugin(&action.name)
        .ok_or_else(|| AppError::NotFound("Plugin not found".into()))?;

    match action.action.as_str() {
        "install" => plugin.install(&pool).await?,
        "uninstall" => plugin.uninstall(&pool).await?,
        "enable" => plugin.enable(&pool).await?,
        "disable" => plugin.disable(&pool).await?,
        _ => return Err(AppError::Validation("Invalid action".into())),
    }

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Operation successful",
        "data": null::<()>
    })))
}

pub async fn update_plugin_config(
    pool: web::Data<sqlx::MySqlPool>,
    config: web::Json<PluginConfig>,
) -> Result<HttpResponse, AppError> {
    let plugin = get_plugin(&config.name)
        .ok_or_else(|| AppError::NotFound("Plugin not found".into()))?;

    let config_key = format!("{}_config", config.name);
    Setting::set(&pool, &config_key, &config.settings.to_string()).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Configuration updated",
        "data": null::<()>
    })))
}

fn get_plugin(name: &str) -> Option<Box<dyn Plugin>> {
    match name {
        "weather" => Some(Box::new(crate::plugins::weather::WeatherPlugin)),
        "poetry" => Some(Box::new(crate::plugins::poetry::PoetryPlugin)),
        "todo" => Some(Box::new(crate::plugins::todo::TodoPlugin)),
        "top_search" => Some(Box::new(crate::plugins::top_search::TopSearchPlugin)),
        _ => None,
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/plugin")
            .route("", web::get().to(list_plugins))
            .route("/action", web::post().to(handle_plugin))
            .route("/config", web::post().to(update_plugin_config))
    );
} 