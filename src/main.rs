use actix_web::{App, HttpServer};
use marktab::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init();

    // 加载配置
    let config = Config::from_env().expect("Failed to load config");

    println!("Server starting at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            // 这里添加路由和中间件配置
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
} 