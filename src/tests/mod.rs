use sqlx::MySqlPool;
use dotenv::dotenv;
use crate::models::user::{User, NewUser};

pub async fn setup_test_db() -> MySqlPool {
    dotenv().ok();
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

pub async fn create_test_user(pool: &MySqlPool) -> User {
    let new_user = NewUser {
        email: "test@example.com".to_string(),
        nickname: "Test User".to_string(),
        password: bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap(),
    };

    User::create(pool, &new_user)
        .await
        .expect("Failed to create test user")
}

pub async fn cleanup_test_db(pool: &MySqlPool) {
    sqlx::query!("DELETE FROM users WHERE email = ?", "test@example.com")
        .execute(pool)
        .await
        .expect("Failed to cleanup test user");
} 