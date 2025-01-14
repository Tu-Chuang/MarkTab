use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::error::AppError;
use super::{WeatherPlugin, WeatherConfig};

#[derive(Deserialize)]
pub struct LocationQuery {
    location: String,
}

pub async fn get_weather(
    pool: web::Data<sqlx::MySqlPool>,
    plugin: web::Data<WeatherPlugin>,
    query: web::Query<LocationQuery>,
) -> Result<HttpResponse, AppError> {
    let weather = plugin.get_weather(&pool, &query.location).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": weather
    })))
}

pub async fn get_config(
    pool: web::Data<sqlx::MySqlPool>,
    plugin: web::Data<WeatherPlugin>,
) -> Result<HttpResponse, AppError> {
    let config = plugin.get_config(&pool).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": config
    })))
}

impl WeatherPlugin {
    pub fn configure_routes(&self, cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/weather")
                .route("/current", web::get().to(get_weather))
                .route("/config", web::get().to(get_config))
        );
    }
} 