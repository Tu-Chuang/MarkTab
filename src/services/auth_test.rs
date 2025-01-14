#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup_test_db, create_test_user, cleanup_test_db};

    #[actix_rt::test]
    async fn test_login_success() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        let result = AuthService::login(
            &pool,
            &user.email,
            "password123",
            "test_agent",
            "127.0.0.1",
        ).await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.access_token.is_empty());
        assert!(!token.refresh_token.is_empty());

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_login_wrong_password() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        let result = AuthService::login(
            &pool,
            &user.email,
            "wrong_password",
            "test_agent",
            "127.0.0.1",
        ).await;

        assert!(result.is_err());
        match result {
            Err(AppError::Auth(msg)) => assert!(msg.contains("Invalid")),
            _ => panic!("Expected Auth error"),
        }

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_refresh_token() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        let token = AuthService::login(
            &pool,
            &user.email,
            "password123",
            "test_agent",
            "127.0.0.1",
        ).await.unwrap();

        let result = AuthService::refresh_token(
            &pool,
            &token.refresh_token,
            "test_agent",
            "127.0.0.1",
        ).await;

        assert!(result.is_ok());
        let new_token = result.unwrap();
        assert!(!new_token.access_token.is_empty());
        assert!(!new_token.refresh_token.is_empty());
        assert_ne!(token.access_token, new_token.access_token);

        cleanup_test_db(&pool).await;
    }
} 