#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::setup_test_db;
    use serde_json::json;

    #[actix_rt::test]
    async fn test_set_and_get_setting() {
        let pool = setup_test_db().await;
        
        let key = "test_setting";
        let value = json!({
            "name": "Test Setting",
            "value": 123
        });

        Setting::set(&pool, key, &value).await.unwrap();

        let found = Setting::get(&pool, key).await.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.key, key);
        assert_eq!(found.value, value);

        // 清理测试数据
        sqlx::query!("DELETE FROM settings WHERE `key` = ?", key)
            .execute(&pool)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_delete_setting() {
        let pool = setup_test_db().await;
        
        let key = "test_setting";
        let value = json!("test value");

        Setting::set(&pool, key, &value).await.unwrap();
        Setting::delete(&pool, key).await.unwrap();

        let found = Setting::get(&pool, key).await.unwrap();
        assert!(found.is_none());
    }

    #[actix_rt::test]
    async fn test_list_settings() {
        let pool = setup_test_db().await;
        
        // 创建多个测试设置
        let test_settings = vec![
            ("test1", json!("value1")),
            ("test2", json!("value2")),
            ("test3", json!("value3")),
        ];

        for (key, value) in &test_settings {
            Setting::set(&pool, key, value).await.unwrap();
        }

        let (settings, total) = Setting::list(&pool, 1, 10).await.unwrap();
        assert!(total >= test_settings.len() as i64);
        assert!(settings.len() >= test_settings.len());

        // 清理测试数据
        for (key, _) in test_settings {
            sqlx::query!("DELETE FROM settings WHERE `key` = ?", key)
                .execute(&pool)
                .await
                .unwrap();
        }
    }
} 