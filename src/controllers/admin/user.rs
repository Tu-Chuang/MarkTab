use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{
    error::AppError,
    models::user::{User, NewUser},
};

#[derive(Deserialize)]
pub struct UserQuery {
    email: Option<String>,
    nickname: Option<String>,
    status: Option<i32>,
    page: Option<u32>,
    per_page: Option<u32>,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    email: Option<String>,
    nickname: Option<String>,
    password: Option<String>,
    status: Option<i32>,
    is_admin: Option<bool>,
}

pub async fn list_users(
    pool: web::Data<sqlx::MySqlPool>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM users WHERE 1=1");
    let mut params = Vec::new();

    if let Some(email) = &query.email {
        sql.push_str(" AND email LIKE ?");
        params.push(format!("%{}%", email));
    }

    if let Some(nickname) = &query.nickname {
        sql.push_str(" AND nickname LIKE ?");
        params.push(format!("%{}%", nickname));
    }

    if let Some(status) = query.status {
        sql.push_str(" AND status = ?");
        params.push(status.to_string());
    }

    sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");
    params.push(per_page.to_string());
    params.push(offset.to_string());

    let users = sqlx::query_as::<_, User>(&sql)
        .bind_all(params)
        .fetch_all(&pool)
        .await?;

    // 获取总数
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": {
            "items": users,
            "total": count,
            "page": page,
            "per_page": per_page
        }
    })))
}

pub async fn update_user(
    pool: web::Data<sqlx::MySqlPool>,
    user_id: web::Path<i32>,
    update: web::Json<UpdateUser>,
) -> Result<HttpResponse, AppError> {
    let mut user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = ?",
        user_id.into_inner()
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    if let Some(email) = &update.email {
        user.email = email.clone();
    }
    if let Some(nickname) = &update.nickname {
        user.nickname = nickname.clone();
    }
    if let Some(password) = &update.password {
        user.password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    }
    if let Some(status) = update.status {
        user.status = status;
    }
    if let Some(is_admin) = update.is_admin {
        user.is_admin = is_admin;
    }

    sqlx::query!(
        r#"
        UPDATE users 
        SET email = ?, nickname = ?, password = ?, status = ?, is_admin = ?
        WHERE id = ?
        "#,
        user.email,
        user.nickname,
        user.password,
        user.status,
        user.is_admin,
        user.id
    )
    .execute(&pool)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "User updated",
        "data": user
    })))
}

pub async fn delete_user(
    pool: web::Data<sqlx::MySqlPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    sqlx::query!("DELETE FROM users WHERE id = ?", user_id.into_inner())
        .execute(&pool)
        .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "User deleted",
        "data": null::<()>
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("", web::get().to(list_users))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
    );
} 