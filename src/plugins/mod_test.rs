#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::setup_test_db;
    use actix_web::web;

    #[actix_rt::test]
    async fn test_plugin_registry() {
        let pool = setup_test_db().await;
        let registry = PluginRegistry::new();

        // 注册测试插件
        registry.register(Box::new(TestPlugin));

        // 验证插件信息
        let plugins = registry.list_plugins();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].info().name_en, "test");

        // 测试插件启用/禁用
        registry.enable_plugin(&pool, "test").await.unwrap();
        assert!(registry.is_plugin_enabled("test"));

        registry.disable_plugin(&pool, "test").await.unwrap();
        assert!(!registry.is_plugin_enabled("test"));
    }

    struct TestPlugin;

    #[async_trait]
    impl Plugin for TestPlugin {
        fn info(&self) -> PluginInfo {
            PluginInfo {
                name: "测试插件".to_string(),
                name_en: "test".to_string(),
                version: "1.0.0".to_string(),
                description: "测试用插件".to_string(),
                author: "Test Author".to_string(),
                settings: None,
            }
        }

        async fn install(&self, pool: &MySqlPool) -> Result<(), AppError> {
            sqlx::query!(
                r#"
                CREATE TABLE IF NOT EXISTS plugin_test (
                    id INT PRIMARY KEY AUTO_INCREMENT,
                    name VARCHAR(255) NOT NULL,
                    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                )
                "#
            )
            .execute(pool)
            .await?;
            Ok(())
        }

        async fn uninstall(&self, pool: &MySqlPool) -> Result<(), AppError> {
            sqlx::query!("DROP TABLE IF EXISTS plugin_test")
                .execute(pool)
                .await?;
            Ok(())
        }

        fn configure_routes(&self, cfg: &mut web::ServiceConfig) {
            cfg.service(
                web::scope("/test")
                    .route("", web::get().to(|| async { "test plugin" }))
            );
        }
    }
} 