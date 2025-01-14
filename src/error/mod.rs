use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let error_response = serde_json::json!({
            "code": 0,
            "msg": self.to_string(),
            "data": null::<()>
        });

        actix_web::HttpResponse::Ok()
            .json(error_response)
    }
} 