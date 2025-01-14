use actix_web::{web, HttpResponse};
use crate::{error::AppError, models::metrics::SystemMetrics};

pub async fn get_metrics(
    pool: web::Data<sqlx::MySqlPool>,
) -> Result<HttpResponse, AppError> {
    let metrics = SystemMetrics::get_latest(&pool).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": metrics
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/metrics")
            .route("", web::get().to(get_metrics))
    );
} 