use actix_web::{web, HttpResponse, HttpRequest};
use serde::Deserialize;
use crate::error::AppError;
use super::models::{Poetry, PoetryFavorite};

#[derive(Deserialize)]
pub struct SearchQuery {
    keyword: String,
    page: Option<u32>,
    per_page: Option<u32>,
}

#[derive(Deserialize)]
pub struct PageQuery {
    page: Option<u32>,
    per_page: Option<u32>,
}

pub async fn random(
    pool: web::Data<sqlx::MySqlPool>,
) -> Result<HttpResponse, AppError> {
    let poetry = Poetry::random(&pool).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": poetry
    })))
}

pub async fn search(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let (poems, total) = Poetry::search(
        &pool,
        &query.keyword,
        page,
        per_page,
    ).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": {
            "items": poems,
            "total": total,
            "page": page,
            "per_page": per_page
        }
    })))
}

pub async fn add_favorite(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    poetry_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or_else(|| AppError::Auth("Login required".into()))?;

    // 检查诗词是否存在
    let poetry = Poetry::find_by_id(&pool, poetry_id.into_inner()).await?
        .ok_or_else(|| AppError::NotFound("Poetry not found".into()))?;

    let favorite = PoetryFavorite::create(
        &pool,
        user.id,
        poetry.id,
    ).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Added to favorites",
        "data": favorite
    })))
}

pub async fn remove_favorite(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    poetry_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or_else(|| AppError::Auth("Login required".into()))?;

    PoetryFavorite::delete(
        &pool,
        user.id,
        poetry_id.into_inner(),
    ).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Removed from favorites",
        "data": null::<()>
    })))
}

pub async fn list_favorites(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or_else(|| AppError::Auth("Login required".into()))?;

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let (poems, total) = PoetryFavorite::list_by_user(
        &pool,
        user.id,
        page,
        per_page,
    ).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": {
            "items": poems,
            "total": total,
            "page": page,
            "per_page": per_page
        }
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/poetry")
            .route("/random", web::get().to(random))
            .route("/search", web::get().to(search))
            .route("/favorite/{id}", web::post().to(add_favorite))
            .route("/favorite/{id}", web::delete().to(remove_favorite))
            .route("/favorites", web::get().to(list_favorites))
    );
} 