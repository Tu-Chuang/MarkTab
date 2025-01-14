use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::error::AppError;
use super::models::TopSearchItem;

#[derive(Deserialize)]
pub struct PlatformQuery {
    platform: String,
    limit: Option<i32>,
}

pub async fn get_hot_list(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<PlatformQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(50);
    let items = TopSearchItem::list_by_platform(
        &pool,
        &query.platform,
        limit,
    ).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": items
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/top_search")
            .route("", web::get().to(get_hot_list))
    );
} 