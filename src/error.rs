use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authorization error: {0}")]
    Forbidden(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Plugin not found")]
    PluginNotFound,

    #[error("Plugin already exists")]
    PluginAlreadyExists,

    #[error("Plugin operation failed: {0}")]
    PluginError(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_code) = match self {
            AppError::Auth(_) => (401, 401),
            AppError::Forbidden(_) => (403, 403),
            AppError::Validation(_) => (400, 400),
            AppError::NotFound(_) => (404, 404),
            _ => (500, 500),
        };

        HttpResponse::build(status.into())
            .json(json!({
                "code": 0,
                "msg": self.to_string(),
                "error_code": error_code,
                "data": null::<()>
            }))
    }
}

pub type AppResult<T> = Result<T, AppError>; 