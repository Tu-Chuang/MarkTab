#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup_test_db, cleanup_test_db};

    #[actix_rt::test]
    async fn test_create_user() {
        let pool = setup_test_db().await;
        
        let new_user = NewUser {
            email: "test@example.com".to_string(),
            nickname: "Test User".to_string(),
            password: bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap(),
        };

        let user = User::create(&pool, &new_user).await.unwrap();
        assert_eq!(user.email, new_user.email);
        assert_eq!(user.nickname, new_user.nickname);
        assert!(!user.is_admin);
        assert_eq!(user.status, 1);

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_find_by_email() {
        let pool = setup_test_db().await;
        
        let new_user = NewUser {
            email: "test@example.com".to_string(),
            nickname: "Test User".to_string(),
            password: bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap(),
        };

        let created_user = User::create(&pool, &new_user).await.unwrap();
        let found_user = User::find_by_email(&pool, &new_user.email).await.unwrap();

        assert!(found_user.is_some());
        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.email, new_user.email);

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_verify_password() {
        let pool = setup_test_db().await;
        
        let password = "password123";
        let new_user = NewUser {
            email: "test@example.com".to_string(),
            nickname: "Test User".to_string(),
            password: bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap(),
        };

        let user = User::create(&pool, &new_user).await.unwrap();
        assert!(user.verify_password(password));
        assert!(!user.verify_password("wrong_password"));

        cleanup_test_db(&pool).await;
    }
} 