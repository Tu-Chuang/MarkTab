#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup_test_db, create_test_user, cleanup_test_db};

    #[actix_rt::test]
    async fn test_todo_plugin() {
        let pool = setup_test_db().await;
        let plugin = TodoPlugin;

        // 测试插件安装
        plugin.install(&pool).await.unwrap();

        // 创建测试用户和文件夹
        let user = create_test_user(&pool).await;
        let folder = sqlx::query_as!(
            Folder,
            r#"
            INSERT INTO plugin_todo_folders (user_id, name)
            VALUES (?, ?)
            RETURNING *
            "#,
            user.id,
            "Test Folder"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        // 创建待办事项
        let todo = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO plugin_todos (user_id, folder_id, content)
            VALUES (?, ?, ?)
            RETURNING *
            "#,
            user.id,
            folder.id,
            "Test Todo Item"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        // 验证数据
        assert_eq!(folder.name, "Test Folder");
        assert_eq!(todo.content, "Test Todo Item");
        assert!(!todo.completed);

        // 测试插件卸载
        plugin.uninstall(&pool).await.unwrap();

        // 验证表已删除
        let result = sqlx::query!("SHOW TABLES LIKE 'plugin_todos'")
            .fetch_optional(&pool)
            .await
            .unwrap();
        assert!(result.is_none());

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_plugin_info() {
        let plugin = TodoPlugin;
        let info = plugin.info();

        assert_eq!(info.name_en, "todo");
        assert_eq!(info.version, "1.0.0");
        assert!(!info.name.is_empty());
        assert!(!info.description.is_empty());
        assert!(!info.author.is_empty());
    }
} 