use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{
    error::AppResult,
    middleware::auth::AuthMiddleware,
    models::user::User,
    utils::Response,
};

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub email: String,
    pub nickname: String,
    pub is_admin: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

pub async fn get_profile(user: web::ReqData<User>) -> AppResult<HttpResponse> {
    let user_info = UserInfo {
        id: user.id,
        email: user.email.clone(),
        nickname: user.nickname.clone(),
        is_admin: user.is_admin,
        created_at: user.created_at,
    };

    Ok(HttpResponse::Ok().json(Response::success(user_info)))
}

pub async fn update_profile(
    pool: web::Data<sqlx::MySqlPool>,
    user: web::ReqData<User>,
    req: web::Json<UpdateProfileRequest>,
) -> AppResult<HttpResponse> {
    sqlx::query!(
        "UPDATE MARKTAB_users SET nickname = ? WHERE id = ?",
        req.nickname,
        user.id
    )
    .execute(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(Response::success(())))
}

pub async fn change_password(
    pool: web::Data<sqlx::MySqlPool>,
    user: web::ReqData<User>,
    req: web::Json<ChangePasswordRequest>,
) -> AppResult<HttpResponse> {
    // 验证旧密码
    let valid = bcrypt::verify(&req.old_password, &user.password)?;
    if !valid {
        return Err(crate::error::AppError::Auth("Invalid old password".to_string()));
    }

    // 加密新密码
    let hashed = bcrypt::hash(&req.new_password, bcrypt::DEFAULT_COST)?;

    // 更新密码
    sqlx::query!(
        "UPDATE MARKTAB_users SET password = ? WHERE id = ?",
        hashed,
        user.id
    )
    .execute(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(Response::success(())))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(AuthMiddleware)
            .route("/profile", web::get().to(get_profile))
            .route("/profile", web::put().to(update_profile))
            .route("/password", web::put().to(change_password))
    );
} 