use actix_web::web;
mod user;
mod plugin;
mod setting;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap(crate::middleware::admin::AdminAuth)
            .configure(user::config)
            .configure(plugin::config)
            .configure(setting::config)
    );
} 