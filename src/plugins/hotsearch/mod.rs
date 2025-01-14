fn info(&self) -> PluginInfo {
    PluginInfo {
        name: "MARKTAB热搜".to_string(),
        name_en: "hotsearch".to_string(),
        version: "1.0.0".to_string(),
        description: "热搜聚合服务".to_string(),
        author: "MARKTAB Team".to_string(),
        settings: None,
    }
}

async fn install(&self, pool: &MySqlPool) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS plugin_hotsearch (
            id INT PRIMARY KEY AUTO_INCREMENT,
            platform VARCHAR(255) NOT NULL,
            title VARCHAR(255) NOT NULL,
            url VARCHAR(1024) NOT NULL,
            rank INT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
} 