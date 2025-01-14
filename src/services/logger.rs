use actix_web::HttpRequest;
use tracing::{Level, Event, Subscriber};
use tracing_subscriber::Layer;
use crate::{error::AppError, models::log::SystemLog};

pub struct DatabaseLogger {
    pool: sqlx::MySqlPool,
}

impl DatabaseLogger {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn log(
        &self,
        level: &str,
        module: &str,
        message: &str,
        req: Option<&HttpRequest>,
    ) -> Result<(), AppError> {
        let user_id = req.and_then(|r| r.extensions().get::<User>())
            .map(|u| u.id);
        
        let ip = req.map(|r| r.peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string()));
            
        let user_agent = req.and_then(|r| r.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok()));

        SystemLog::create(
            &self.pool,
            level,
            module,
            message,
            user_id,
            ip.as_deref(),
            user_agent,
        ).await?;

        Ok(())
    }
}

impl<S: Subscriber> Layer<S> for DatabaseLogger {
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let level = event.metadata().level();
        let module = event.metadata().module_path().unwrap_or("unknown");
        let message = format!("{:?}", event);

        tokio::spawn(self.log(
            level.as_str(),
            module,
            &message,
            None,
        ));
    }
} 