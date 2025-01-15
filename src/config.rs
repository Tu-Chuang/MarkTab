use crate::error::{AppError, AppResult};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub upload_dir: String,
    pub backup_dir: String,
    pub redis_url: Option<String>,
}

impl Config {
    pub fn from_env() -> AppResult<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| AppError::Internal("DATABASE_URL must be set".to_string()))?,
            
            host: env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| AppError::Internal("Invalid PORT".to_string()))?,
            
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| AppError::Internal("JWT_SECRET must be set".to_string()))?,
            
            jwt_expires_in: env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "7d".to_string()),
            
            upload_dir: env::var("UPLOAD_DIR")
                .unwrap_or_else(|_| "/opt/MARKTAB/uploads".to_string()),
            
            backup_dir: env::var("BACKUP_DIR")
                .unwrap_or_else(|_| "/opt/MARKTAB/backups".to_string()),
            
            redis_url: env::var("REDIS_URL").ok(),
        })
    }
} 