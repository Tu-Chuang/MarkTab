#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup_test_db, create_test_user, cleanup_test_db};
    use chrono::Utc;

    #[actix_rt::test]
    async fn test_create_token() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        let token = Token::create(
            &pool,
            user.id,
            "test_token",
            "test_agent",
            "127.0.0.1",
            Utc::now() + chrono::Duration::hours(1),
        ).await.unwrap();

        assert_eq!(token.user_id, user.id);
        assert_eq!(token.token, "test_token");
        assert_eq!(token.user_agent, "test_agent");
        assert_eq!(token.ip_address, "127.0.0.1");
        assert_eq!(token.status, 1);

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_find_valid_token() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        let token = Token::create(
            &pool,
            user.id,
            "test_token",
            "test_agent",
            "127.0.0.1",
            Utc::now() + chrono::Duration::hours(1),
        ).await.unwrap();

        let found_token = Token::find_valid(&pool, "test_token").await.unwrap();
        assert!(found_token.is_some());
        let found_token = found_token.unwrap();
        assert_eq!(found_token.id, token.id);
        assert_eq!(found_token.user_id, user.id);

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_invalidate_token() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        let token = Token::create(
            &pool,
            user.id,
            "test_token",
            "test_agent",
            "127.0.0.1",
            Utc::now() + chrono::Duration::hours(1),
        ).await.unwrap();

        Token::invalidate(&pool, "test_token").await.unwrap();

        let found_token = Token::find_valid(&pool, "test_token").await.unwrap();
        assert!(found_token.is_none());

        cleanup_test_db(&pool).await;
    }
} 