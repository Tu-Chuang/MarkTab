use actix_web::{web, HttpResponse, HttpRequest};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use chrono::Utc;
use md5::{Md5, Digest};
use crate::{
    error::AppError,
    models::file::File,
    utils::get_content_type,
};

pub async fn upload(
    pool: web::Data<sqlx::MySqlPool>,
    mut payload: Multipart,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // 获取当前用户
    let user = req.extensions().get::<User>().cloned();
    
    // 创建上传目录
    let upload_dir = format!("uploads/{}", Utc::now().format("%Y/%m/%d"));
    std::fs::create_dir_all(&upload_dir)?;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        let filename = content_type
            .get_filename()
            .ok_or_else(|| AppError::Validation("No filename provided".into()))?;

        // 计算文件hash
        let mut hasher = Md5::new();
        let mut size = 0i64;
        let filepath = format!("{}/{}", upload_dir, filename);
        let mut f = std::fs::File::create(&filepath)?;
        
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            hasher.update(&data);
            size += data.len() as i64;
            f.write_all(&data)?;
        }

        let hash = format!("{:x}", hasher.finalize());
        
        // 检查是否已存在相同文件
        if let Some(existing_file) = File::find_by_hash(&pool, &hash).await? {
            std::fs::remove_file(&filepath)?;
            return Ok(HttpResponse::Ok().json(existing_file));
        }

        // 保存文件记录
        let file = File::create(
            &pool,
            user.map(|u| u.id),
            &filepath,
            &get_content_type(&filepath),
            size,
            &hash,
        ).await?;

        return Ok(HttpResponse::Ok().json(file));
    }

    Err(AppError::Validation("No file uploaded".into()))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/file")
            .route("/upload", web::post().to(upload))
    );
} 