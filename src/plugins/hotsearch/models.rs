impl HotSearch {
    pub async fn create_batch(
        pool: &MySqlPool,
        items: &[NewHotSearch],
    ) -> Result<(), AppError> {
        // 先清空旧数据
        sqlx::query!("TRUNCATE TABLE plugin_hotsearch")
            .execute(pool)
            .await?;

        // 批量插入新数据
        for item in items {
            sqlx::query!(
                r#"
                INSERT INTO plugin_hotsearch (platform, title, url, rank)
                VALUES (?, ?, ?, ?)
                "#,
                item.platform,
                item.title,
                item.url,
                item.rank
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    pub async fn list_by_platform(
        pool: &MySqlPool,
        platform: &str,
        limit: u32,
    ) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_hotsearch
            WHERE platform = ?
            ORDER BY rank ASC
            LIMIT ?
            "#,
            platform,
            limit
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_all(
        pool: &MySqlPool,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Self>, i64), AppError> {
        let offset = (page - 1) * per_page;

        let items = sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM plugin_hotsearch
            ORDER BY platform ASC, rank ASC
            LIMIT ? OFFSET ?
            "#,
            per_page,
            offset
        )
        .fetch_all(pool)
        .await?;

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM plugin_hotsearch"
        )
        .fetch_one(pool)
        .await?;

        Ok((items, total.unwrap_or(0)))
    }
} 