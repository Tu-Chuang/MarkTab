use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::{error::AppError, models::setting::Setting};

pub async fn get_site_config(
    pool: web::Data<sqlx::MySqlPool>,
) -> Result<HttpResponse, AppError> {
    let config = json!({
        "email": Setting::get(&pool, "email").await?,
        "qqGroup": Setting::get(&pool, "qqGroup").await?,
        "beianMps": Setting::get(&pool, "beianMps").await?,
        "copyright": Setting::get(&pool, "copyright").await?,
        "recordNumber": Setting::get(&pool, "recordNumber").await?,
        "mobileRecordNumber": Setting::get(&pool, "mobileRecordNumber").await?.unwrap_or("0".to_string()),
        "logo": Setting::get(&pool, "logo").await?,
        "qq_login": Setting::get(&pool, "qq_login").await?.unwrap_or("0".to_string()),
        "user_register": Setting::get(&pool, "user_register").await?.unwrap_or("0".to_string()),
    });

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": config
    })))
}

pub async fn get_background(
    pool: web::Data<sqlx::MySqlPool>,
) -> Result<HttpResponse, AppError> {
    let background = Setting::get(&pool, "background").await?
        .unwrap_or_else(|| "static/background.jpeg".to_string());

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": {
            "background": background,
            "mime": 0
        }
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/site")
            .route("/config", web::get().to(get_site_config))
            .route("/background", web::get().to(get_background))
    );
} 