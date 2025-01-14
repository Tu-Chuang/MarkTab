use sqlx::MySqlPool;
use crate::error::AppError;

pub async fn run_migrations(pool: &MySqlPool) -> Result<(), AppError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Database(e))
} 