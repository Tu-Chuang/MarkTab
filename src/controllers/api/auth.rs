use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use crate::{
    error::AppError,
    services::auth::AuthService,
    models::user::{User, NewUser},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    login_req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let user_agent = req.headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let ip = req.peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let token = AuthService::login(
        &pool,
        &login_req.email,
        &login_req.password,
        &user_agent,
        &ip,
    ).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Login successful",
        "data": token
    })))
}

pub async fn register(
    pool: web::Data<sqlx::MySqlPool>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    let hashed_password = bcrypt::hash(&new_user.password, bcrypt::DEFAULT_COST)?;
    
    let user = User::create(&pool, &NewUser {
        password: hashed_password,
        ..new_user.into_inner()
    }).await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Registration successful",
        "data": user
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
    );
} 