use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::{error::AppError, models::log::SystemLog};

#[derive(Deserialize)]
pub struct LogQuery {
    page: Option<u32>,
    per_page: Option<u32>,
}

pub async fn list_logs(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<LogQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let (logs, total) = SystemLog::list(&pool, page, per_page).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": {
            "items": logs,
            "total": total,
            "page": page,
            "per_page": per_page
        }
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/log")
            .route("", web::get().to(list_logs))
    );
} 