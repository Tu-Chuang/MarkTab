use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{
    error::AppResult,
    services::auth::AuthService,
    utils::Response,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

pub async fn login(
    pool: web::Data<sqlx::MySqlPool>,
    req: web::Json<LoginRequest>,
    user_agent: web::Header<actix_web::http::header::UserAgent>,
    client_ip: web::Header<actix_web::http::header::Host>,
) -> AppResult<HttpResponse> {
    let token = AuthService::login(
        &pool,
        &req.email,
        &req.password,
        user_agent.as_str(),
        client_ip.as_str(),
    ).await?;

    Ok(HttpResponse::Ok().json(Response::success(token)))
}

pub async fn refresh(
    pool: web::Data<sqlx::MySqlPool>,
    token: web::Header<actix_web::http::header::Authorization>,
) -> AppResult<HttpResponse> {
    let new_token = AuthService::refresh_token(&pool, token.as_str()).await?;
    
    Ok(HttpResponse::Ok().json(Response::success(new_token)))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/refresh", web::post().to(refresh))
    );
} 