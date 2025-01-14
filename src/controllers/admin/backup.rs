use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::{error::AppError, services::backup::BackupService};

#[derive(Deserialize)]
pub struct PageQuery {
    page: Option<u32>,
    per_page: Option<u32>,
}

pub async fn create_backup(
    pool: web::Data<sqlx::MySqlPool>,
) -> Result<HttpResponse, AppError> {
    let backup_service = BackupService::new(
        pool.get_ref().clone(),
        "backups".to_string(),
    );
    
    let backup_id = backup_service.create_backup().await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Backup started",
        "data": backup_id
    })))
}

pub async fn list_backups(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AppError> {
    let backup_service = BackupService::new(
        pool.get_ref().clone(),
        "backups".to_string(),
    );

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let (backups, total) = backup_service.list_backups(page, per_page).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": {
            "items": backups,
            "total": total,
            "page": page,
            "per_page": per_page
        }
    })))
}

pub async fn delete_backup(
    pool: web::Data<sqlx::MySqlPool>,
    backup_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let backup_service = BackupService::new(
        pool.get_ref().clone(),
        "backups".to_string(),
    );

    backup_service.delete_backup(backup_id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Backup deleted",
        "data": null::<()>
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/backup")
            .route("", web::post().to(create_backup))
            .route("", web::get().to(list_backups))
            .route("/{id}", web::delete().to(delete_backup))
    );
} 