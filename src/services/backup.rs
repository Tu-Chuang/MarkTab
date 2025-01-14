use std::{fs, path::Path, process::Command};
use chrono::{Utc, DateTime};
use sqlx::MySqlPool;
use sha2::{Sha256, Digest};
use crate::error::AppError;

#[derive(Debug)]
pub struct BackupService {
    pool: MySqlPool,
    backup_dir: String,
}

impl BackupService {
    pub fn new(pool: MySqlPool, backup_dir: String) -> Self {
        Self { pool, backup_dir }
    }

    pub async fn create_backup(&self) -> Result<i32, AppError> {
        // 创建备份记录
        let filename = format!("backup_{}.sql.gz", Utc::now().format("%Y%m%d_%H%M%S"));
        let backup_id = sqlx::query_scalar!(
            r#"
            INSERT INTO system_backups (filename, size, hash, backup_type, status)
            VALUES (?, 0, '', 'full', 'pending')
            "#,
            filename
        )
        .execute(&self.pool)
        .await?
        .last_insert_id() as i32;

        // 更新状态为运行中
        sqlx::query!(
            r#"
            UPDATE system_backups 
            SET status = 'running', started_at = NOW()
            WHERE id = ?
            "#,
            backup_id
        )
        .execute(&self.pool)
        .await?;

        // 确保备份目录存在
        let backup_path = Path::new(&self.backup_dir);
        if !backup_path.exists() {
            fs::create_dir_all(backup_path)?;
        }

        // 执行备份命令
        let output = Command::new("mysqldump")
            .arg("--single-transaction")
            .arg("--quick")
            .arg("--lock-tables=false")
            .arg(format!("--result-file={}/{}", self.backup_dir, filename))
            .arg("--databases")
            .arg("MARKTAB")
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            sqlx::query!(
                r#"
                UPDATE system_backups 
                SET status = 'failed', 
                    error_message = ?,
                    completed_at = NOW()
                WHERE id = ?
                "#,
                error.to_string(),
                backup_id
            )
            .execute(&self.pool)
            .await?;

            return Err(AppError::Internal(error.to_string()));
        }

        // 压缩备份文件
        Command::new("gzip")
            .arg("-f")
            .arg(format!("{}/{}", self.backup_dir, filename))
            .output()?;

        // 计算文件大小和哈希
        let backup_file = format!("{}/{}.gz", self.backup_dir, filename);
        let metadata = fs::metadata(&backup_file)?;
        let mut file = fs::File::open(&backup_file)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        let hash = format!("{:x}", hasher.finalize());

        // 更新备份记录
        sqlx::query!(
            r#"
            UPDATE system_backups 
            SET status = 'completed',
                size = ?,
                hash = ?,
                completed_at = NOW()
            WHERE id = ?
            "#,
            metadata.len() as i64,
            hash,
            backup_id
        )
        .execute(&self.pool)
        .await?;

        Ok(backup_id)
    }

    pub async fn list_backups(
        &self,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<SystemBackup>, i64), AppError> {
        let offset = (page - 1) * per_page;
        
        let backups = sqlx::query_as!(
            SystemBackup,
            r#"
            SELECT * FROM system_backups
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            per_page,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM system_backups"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((backups, total.unwrap_or(0)))
    }

    pub async fn delete_backup(&self, id: i32) -> Result<(), AppError> {
        let backup = sqlx::query_as!(
            SystemBackup,
            "SELECT * FROM system_backups WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Backup not found".into()))?;

        // 删除文件
        let backup_file = format!("{}/{}", self.backup_dir, backup.filename);
        if Path::new(&backup_file).exists() {
            fs::remove_file(backup_file)?;
        }

        // 删除记录
        sqlx::query!("DELETE FROM system_backups WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SystemBackup {
    pub id: i32,
    pub filename: String,
    pub size: i64,
    pub hash: String,
    pub backup_type: String,
    pub status: String,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
} 