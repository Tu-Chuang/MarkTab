use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use serde::Serialize;
use crate::{
    error::AppResult,
    middleware::auth::AuthMiddleware,
    models::user::User,
    services::file::FileService,
    utils::Response,
};

#[derive(Debug, Serialize)]
pub struct FileInfo {
    pub id: i32,
    pub filename: String,
    pub mime_type: String,
    pub size: i64,
    pub path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn upload(
    pool: web::Data<sqlx::MySqlPool>,
    user: web::ReqData<User>,
    mut payload: Multipart,
) -> AppResult<HttpResponse> {
    let mut files = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let filename = field.content_disposition()
            .get_filename()
            .ok_or_else(|| crate::error::AppError::Validation("Missing filename".to_string()))?
            .to_string();

        let mime_type = field.content_type().to_string();
        let mut size = 0i64;
        let mut file_data = Vec::new();

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            size += data.len() as i64;
            file_data.extend_from_slice(&data);
        }

        let file = FileService::save_file(
            &pool,
            user.id,
            &filename,
            &mime_type,
            size,
            file_data,
        ).await?;

        files.push(file);
    }

    Ok(HttpResponse::Ok().json(Response::success(files)))
}

pub async fn list(
    pool: web::Data<sqlx::MySqlPool>,
    user: web::ReqData<User>,
    query: web::Query<ListQuery>,
) -> AppResult<HttpResponse> {
    let (files, total) = FileService::list_files(
        &pool,
        user.id,
        query.page.unwrap_or(1),
        query.per_page.unwrap_or(10),
    ).await?;

    Ok(HttpResponse::Ok().json(Response::success(ListResponse {
        items: files,
        total,
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(10),
    })))
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub items: Vec<FileInfo>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/file")
            .wrap(AuthMiddleware)
            .route("/upload", web::post().to(upload))
            .route("/list", web::get().to(list))
    );
} 