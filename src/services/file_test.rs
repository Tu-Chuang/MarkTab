#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup_test_db, create_test_user, cleanup_test_db};
    use std::fs;
    use actix_web::web::Bytes;

    #[actix_rt::test]
    async fn test_save_file() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;
        let file_service = FileService::new("test_uploads".to_string());

        // 创建测试文件内容
        let content = Bytes::from_static(b"test file content");
        let filename = "test.txt";
        let mime_type = "text/plain";

        // 保存文件
        let file = file_service.save_file(
            &pool,
            user.id,
            filename,
            mime_type,
            content.len() as i64,
            content,
        ).await.unwrap();

        // 验证文件是否正确保存
        assert_eq!(file.filename, filename);
        assert_eq!(file.mime_type, mime_type);
        assert_eq!(file.user_id, user.id);
        assert!(std::path::Path::new(&file.path).exists());

        // 验证文件内容
        let saved_content = fs::read_to_string(&file.path).unwrap();
        assert_eq!(saved_content, "test file content");

        // 清理测试文件和目录
        fs::remove_dir_all("test_uploads").unwrap();
        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    async fn test_delete_file() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await;
        let file_service = FileService::new("test_uploads".to_string());

        // 创建测试文件
        let content = Bytes::from_static(b"test file content");
        let file = file_service.save_file(
            &pool,
            user.id,
            "test.txt",
            "text/plain",
            content.len() as i64,
            content,
        ).await.unwrap();

        // 删除文件
        file_service.delete_file(&pool, file.id).await.unwrap();

        // 验证文件是否已删除
        assert!(!std::path::Path::new(&file.path).exists());
        let found = File::find_by_id(&pool, file.id).await.unwrap();
        assert!(found.is_none());

        // 清理测试目录
        fs::remove_dir_all("test_uploads").unwrap();
        cleanup_test_db(&pool).await;
    }
} 