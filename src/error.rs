use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: i32,
    msg: String,
    error_code: i32,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_code, message) = match self {
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, 401, msg),
            AppError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, 500, &err.to_string()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, 400, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
        };

        HttpResponse::build(status).json(ErrorResponse {
            code: 0,
            msg: message.to_string(),
            error_code,
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::Auth(_) => StatusCode::UNAUTHORIZED,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
} 