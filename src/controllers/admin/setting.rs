use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::{
    error::AppError,
    models::setting::Setting,
};

#[derive(Deserialize)]
pub struct UpdateSetting {
    key: String,
    value: String,
}

pub async fn get_settings(
    pool: web::Data<sqlx::MySqlPool>,
) -> Result<HttpResponse, AppError> {
    let settings = sqlx::query_as!(
        Setting,
        "SELECT * FROM settings ORDER BY `key`"
    )
    .fetch_all(&pool)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": settings
    })))
}

pub async fn update_setting(
    pool: web::Data<sqlx::MySqlPool>,
    setting: web::Json<UpdateSetting>,
) -> Result<HttpResponse, AppError> {
    Setting::set(&pool, &setting.key, &setting.value).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Setting updated",
        "data": null::<()>
    })))
}

pub async fn delete_setting(
    pool: web::Data<sqlx::MySqlPool>,
    key: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    sqlx::query!("DELETE FROM settings WHERE `key` = ?", key.as_str())
        .execute(&pool)
        .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Setting deleted",
        "data": null::<()>
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/setting")
            .route("", web::get().to(get_settings))
            .route("", web::post().to(update_setting))
            .route("/{key}", web::delete().to(delete_setting))
    );
} 