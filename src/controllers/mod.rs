mod auth;
mod file;
mod plugin;
mod user;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(auth::config)
            .configure(file::config)
            .configure(plugin::config)
            .configure(user::config)
    );
}

// Re-export commonly used types
pub use auth::{LoginRequest, TokenResponse};
pub use file::FileInfo;
pub use plugin::PluginInfo;
pub use user::UserInfo; 