use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use crate::error::AppResult;

pub async fn health_check(pool: web::Data<MySqlPool>) -> AppResult<HttpResponse> {
    // 检查数据库连接
    sqlx::query("SELECT 1").execute(pool.get_ref()).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
} 