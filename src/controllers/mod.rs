use actix_web::web;

mod base;
mod admin;
mod api;
mod apps;

pub mod auth;
pub mod user;
pub mod file;
pub mod plugin;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(admin::config)
            .configure(api::config)
            .configure(apps::config)
    );

    auth::config(cfg);
    user::config(cfg);
    file::config(cfg);
    plugin::config(cfg);
} 