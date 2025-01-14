use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use crate::{
    plugins::PluginRegistry,
    middleware::{error::ErrorHandler, metrics::MetricsMiddleware},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod controllers;
mod models;
mod services;
mod utils;
mod config;
mod error;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::login,
        auth::refresh,
        user::get_profile,
        file::upload,
        file::list,
        plugin::list,
        plugin::enable,
        plugin::disable,
        todo::create_folder,
        todo::list_folders,
        todo::create_todo,
        todo::list_todos,
        poetry::random,
        poetry::search,
    ),
    components(
        schemas(
            User, LoginRequest, TokenResponse, Error,
            FileInfo, PluginInfo,
            TodoFolder, TodoItem, Poetry
        )
    ),
    tags(
        (name = "认证", description = "认证相关接口"),
        (name = "用户", description = "用户相关接口"),
        (name = "文件", description = "文件管理接口"),
        (name = "插件", description = "插件管理接口"),
        (name = "待办事项", description = "待办事项管理接口"),
        (name = "诗词", description = "诗词相关接口")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // 数据库连接
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // 创建插件注册器
    let plugin_registry = web::Data::new(PluginRegistry::new());

    HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandler)
            .wrap(MetricsMiddleware)
            .app_data(web::Data::new(pool.clone()))
            .app_data(plugin_registry.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
            .configure(controllers::config)
            .configure(|cfg| plugin_registry.configure_routes(cfg))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} 