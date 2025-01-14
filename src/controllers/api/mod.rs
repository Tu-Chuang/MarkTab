use actix_web::web;
mod auth;
mod site;
mod config;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(auth::config)
            .configure(site::config)
            .configure(config::config)
    );
} 