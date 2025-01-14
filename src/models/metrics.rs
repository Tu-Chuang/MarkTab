use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use crate::error::AppError;
use sysinfo::{System, SystemExt, CpuExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub id: i32,
    pub cpu_usage: f32,
    pub memory_total: i64,
    pub memory_used: i64,
    pub disk_total: i64,
    pub disk_used: i64,
    pub load_avg_1: f32,
    pub load_avg_5: f32,
    pub load_avg_15: f32,
    pub created_at: DateTime<Utc>,
}

impl SystemMetrics {
    pub async fn collect(pool: &MySqlPool) -> Result<Self, AppError> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let memory_total = sys.total_memory() as i64;
        let memory_used = sys.used_memory() as i64;
        let disk_total = sys.disks().iter()
            .map(|disk| disk.total_space())
            .sum::<u64>() as i64;
        let disk_used = sys.disks().iter()
            .map(|disk| disk.total_space() - disk.available_space())
            .sum::<u64>() as i64;
        let load_avg = sys.load_average();

        let metrics = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO system_metrics (
                cpu_usage, memory_total, memory_used,
                disk_total, disk_used,
                load_avg_1, load_avg_5, load_avg_15
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            cpu_usage,
            memory_total,
            memory_used,
            disk_total,
            disk_used,
            load_avg.one,
            load_avg.five,
            load_avg.fifteen
        )
        .execute(pool)
        .await?;

        Ok(Self {
            id: metrics.last_insert_id() as i32,
            cpu_usage,
            memory_total,
            memory_used,
            disk_total,
            disk_used,
            load_avg_1: load_avg.one,
            load_avg_5: load_avg.five,
            load_avg_15: load_avg.fifteen,
            created_at: Utc::now(),
        })
    }

    pub async fn get_latest(pool: &MySqlPool) -> Result<Self, AppError> {
        let metrics = sqlx::query_as!(
            Self,
            "SELECT * FROM system_metrics ORDER BY created_at DESC LIMIT 1"
        )
        .fetch_one(pool)
        .await?;

        Ok(metrics)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiMetrics {
    pub id: i32,
    pub path: String,
    pub method: String,
    pub status_code: i32,
    pub response_time: i32,
    pub created_at: DateTime<Utc>,
}

impl ApiMetrics {
    pub async fn record(
        pool: &MySqlPool,
        path: &str,
        method: &str,
        status_code: i32,
        response_time: i32,
    ) -> Result<Self, AppError> {
        let metrics = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO api_metrics (path, method, status_code, response_time)
            VALUES (?, ?, ?, ?)
            "#,
            path,
            method,
            status_code,
            response_time
        )
        .execute(pool)
        .await?;

        Ok(Self {
            id: metrics.last_insert_id() as i32,
            path: path.to_string(),
            method: method.to_string(),
            status_code,
            response_time,
            created_at: Utc::now(),
        })
    }
} 