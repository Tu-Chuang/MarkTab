use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{
    error::AppResult,
    plugins::PluginRegistry,
    middleware::auth::AuthenticatedUser,
};

#[derive(Debug, Serialize)]
pub struct PluginResponse {
    pub name: String,
    pub name_en: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enabled: bool,
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: serde_json::Value,
}

pub async fn list_plugins(
    registry: web::Data<PluginRegistry>,
    _user: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    let plugins = registry.list_plugins();
    let response: Vec<PluginResponse> = plugins
        .into_iter()
        .map(|info| PluginResponse {
            name: info.name,
            name_en: info.name_en.clone(),
            version: info.version,
            description: info.description,
            author: info.author,
            enabled: registry.is_plugin_enabled(&info.name_en),
            settings: info.settings,
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": response
    })))
}

pub async fn enable_plugin(
    registry: web::Data<PluginRegistry>,
    pool: web::Data<sqlx::MySqlPool>,
    name: web::Path<String>,
    _user: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    registry.enable_plugin(&pool, &name).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": null::<()>
    })))
}

pub async fn disable_plugin(
    registry: web::Data<PluginRegistry>,
    pool: web::Data<sqlx::MySqlPool>,
    name: web::Path<String>,
    _user: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    registry.disable_plugin(&pool, &name).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": null::<()>
    })))
}

pub async fn update_settings(
    registry: web::Data<PluginRegistry>,
    pool: web::Data<sqlx::MySqlPool>,
    name: web::Path<String>,
    req: web::Json<UpdateSettingsRequest>,
    _user: AuthenticatedUser,
) -> AppResult<HttpResponse> {
    // 更新插件配置
    sqlx::query!(
        r#"
        INSERT INTO MARKTAB_plugin_status (name, settings)
        VALUES (?, ?)
        ON DUPLICATE KEY UPDATE
        settings = VALUES(settings)
        "#,
        name.as_str(),
        req.settings.to_string()
    )
    .execute(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": null::<()>
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/plugin")
            .route("", web::get().to(list_plugins))
            .route("/{name}/enable", web::post().to(enable_plugin))
            .route("/{name}/disable", web::post().to(disable_plugin))
            .route("/{name}/settings", web::put().to(update_settings))
    );
} 