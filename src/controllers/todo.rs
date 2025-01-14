use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    user_id: i32,
    title: String,
    completed: bool,
}

pub async fn create_todo(
    pool: web::Data<MySqlPool>,
    todo: web::Json<Todo>,
) -> Result<HttpResponse, AppError> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (user_id, title, completed)
        VALUES (?, ?, ?)
        RETURNING *
        "#,
        todo.user_id,
        todo.title,
        todo.completed
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(todo))
}

pub async fn get_todos(
    pool: web::Data<MySqlPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT * FROM todos WHERE user_id = ?",
        user_id.into_inner()
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(todos))
} 