use std::{fs, path::Path};
use actix_web::web::Bytes;
use chrono::Utc;
use md5::{Md5, Digest};
use crate::{
    error::AppResult,
    models::file::File,
};

pub struct FileService {
    upload_dir: String,
}

impl FileService {
    pub fn new(upload_dir: String) -> Self {
        Self { upload_dir }
    }

    pub async fn save_file(
        &self,
        pool: &sqlx::MySqlPool,
        user_id: i32,
        filename: &str,
        mime_type: &str,
        size: i64,
        content: Bytes,
    ) -> AppResult<File> {
        // 创建上传目录
        let date = Utc::now();
        let dir = format!("{}/{}/{}/{}", 
            self.upload_dir,
            date.format("%Y"),
            date.format("%m"),
            date.format("%d")
        );
        fs::create_dir_all(&dir)?;

        // 计算文件hash
        let mut hasher = Md5::new();
        hasher.update(&content);
        let hash = format!("{:x}", hasher.finalize());

        // 检查是否已存在相同文件
        if let Some(existing) = File::find_by_hash(pool, &hash).await? {
            return Ok(existing);
        }

        // 保存文件
        let ext = Path::new(filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let filepath = format!("{}/{}_{}.{}", dir, hash, date.timestamp(), ext);
        fs::write(&filepath, content)?;

        // 创建文件记录
        File::create(
            pool,
            user_id,
            filename,
            mime_type,
            size,
            &hash,
            &filepath,
        ).await
    }

    pub async fn delete_file(
        &self,
        pool: &sqlx::MySqlPool,
        id: i32,
    ) -> AppResult<()> {
        // 获取文件信息
        let file = File::find_by_id(pool, id)
            .await?
            .ok_or_else(|| crate::error::AppError::NotFound("File not found".into()))?;

        // 删除物理文件
        if Path::new(&file.path).exists() {
            fs::remove_file(&file.path)?;
        }

        // 删除数据库记录
        sqlx::query!("DELETE FROM MARKTAB_files WHERE id = ?", id)
            .execute(pool)
            .await?;

        Ok(())
    }
} 