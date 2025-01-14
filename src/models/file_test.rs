#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup_test_db, create_test_user, cleanup_test_db};
    use std::fs;

    #[actix_rt::test]
    async fn test_create_file() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        let file = File::create(
            &pool,
            user.id,
            "test.txt",
            "text/plain",
            1024,
            "test_hash",
            "test_path",
        ).await.unwrap();

        assert_eq!(file.user_id, user.id);
        assert_eq!(file.filename, "test.txt");
        assert_eq!(file.mime_type, "text/plain");
        assert_eq!(file.size, 1024);
        assert_eq!(file.hash, "test_hash");
        assert_eq!(file.path, "test_path");
        assert_eq!(file.status, 1);

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_find_by_id() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        let file = File::create(
            &pool,
            user.id,
            "test.txt",
            "text/plain",
            1024,
            "test_hash",
            "test_path",
        ).await.unwrap();

        let found = File::find_by_id(&pool, file.id).await.unwrap();
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.id, file.id);
        assert_eq!(found.user_id, user.id);
        assert_eq!(found.filename, "test.txt");

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_list_by_user() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        // 创建多个测试文件
        let test_files = vec![
            ("file1.txt", "text/plain", 1024),
            ("file2.jpg", "image/jpeg", 2048),
            ("file3.pdf", "application/pdf", 4096),
        ];

        for (filename, mime_type, size) in &test_files {
            File::create(
                &pool,
                user.id,
                filename,
                mime_type,
                *size,
                "test_hash",
                "test_path",
            ).await.unwrap();
        }

        let (files, total) = File::list_by_user(&pool, user.id, 1, 10).await.unwrap();
        assert_eq!(total, test_files.len() as i64);
        assert_eq!(files.len(), test_files.len());

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_delete_file() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;

        // 创建测试文件和物理文件
        let test_path = "test_uploads/test.txt";
        fs::create_dir_all("test_uploads").unwrap();
        fs::write(test_path, "test content").unwrap();

        let file = File::create(
            &pool,
            user.id,
            "test.txt",
            "text/plain",
            1024,
            "test_hash",
            test_path,
        ).await.unwrap();

        // 删除文件
        File::delete(&pool, file.id).await.unwrap();

        // 验证数据库记录已删除
        let found = File::find_by_id(&pool, file.id).await.unwrap();
        assert!(found.is_none());

        // 验证物理文件已删除
        assert!(!std::path::Path::new(test_path).exists());

        // 清理测试目录
        fs::remove_dir_all("test_uploads").unwrap();
        cleanup_test_db(&pool).await;
    }
} 