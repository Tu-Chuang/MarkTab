use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFolder {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub folder_id: i32,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub content: Option<String>,
    pub completed: Option<bool>,
}

pub async fn create_folder(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    folder: web::Json<CreateFolder>,
) -> Result<HttpResponse, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or_else(|| AppError::Auth("Login required".into()))?;

    let folder = sqlx::query_as!(
        Folder,
        r#"
        INSERT INTO plugin_todo_folders (user_id, name)
        VALUES (?, ?)
        "#,
        user.id,
        folder.name
    )
    .execute(&pool)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Folder created",
        "data": folder
    })))
}

pub async fn list_folders(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or_else(|| AppError::Auth("Login required".into()))?;

    let folders = sqlx::query_as!(
        Folder,
        "SELECT * FROM plugin_todo_folders WHERE user_id = ?",
        user.id
    )
    .fetch_all(&pool)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": folders
    })))
}

pub async fn create_todo(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    todo: web::Json<CreateTodo>,
) -> Result<HttpResponse, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or_else(|| AppError::Auth("Login required".into()))?;

    // 验证文件夹所有权
    let folder = sqlx::query_as!(
        Folder,
        "SELECT * FROM plugin_todo_folders WHERE id = ? AND user_id = ?",
        todo.folder_id,
        user.id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Folder not found".into()))?;

    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO plugin_todos (user_id, folder_id, content)
        VALUES (?, ?, ?)
        "#,
        user.id,
        folder.id,
        todo.content
    )
    .execute(&pool)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Todo created",
        "data": todo
    })))
}

pub async fn list_todos(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    folder_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or_else(|| AppError::Auth("Login required".into()))?;

    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT * FROM plugin_todos 
        WHERE user_id = ? AND folder_id = ?
        ORDER BY created_at DESC
        "#,
        user.id,
        folder_id.into_inner()
    )
    .fetch_all(&pool)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": todos
    })))
}

pub async fn update_todo(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    todo_id: web::Path<i32>,
    update: web::Json<UpdateTodo>,
) -> Result<HttpResponse, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or_else(|| AppError::Auth("Login required".into()))?;

    let mut todo = sqlx::query_as!(
        Todo,
        "SELECT * FROM plugin_todos WHERE id = ? AND user_id = ?",
        todo_id.into_inner(),
        user.id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Todo not found".into()))?;

    if let Some(content) = &update.content {
        todo.content = content.clone();
    }
    if let Some(completed) = update.completed {
        todo.completed = completed;
    }

    sqlx::query!(
        r#"
        UPDATE plugin_todos 
        SET content = ?, completed = ?
        WHERE id = ?
        "#,
        todo.content,
        todo.completed,
        todo.id
    )
    .execute(&pool)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Todo updated",
        "data": todo
    })))
}

pub async fn delete_todo(
    pool: web::Data<sqlx::MySqlPool>,
    req: HttpRequest,
    todo_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or_else(|| AppError::Auth("Login required".into()))?;

    sqlx::query!(
        "DELETE FROM plugin_todos WHERE id = ? AND user_id = ?",
        todo_id.into_inner(),
        user.id
    )
    .execute(&pool)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "Todo deleted",
        "data": null::<()>
    })))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .route("/folder", web::post().to(create_folder))
            .route("/folder", web::get().to(list_folders))
            .route("/item", web::post().to(create_todo))
            .route("/item/{folder_id}", web::get().to(list_todos))
            .route("/item/{id}", web::put().to(update_todo))
            .route("/item/{id}", web::delete().to(delete_todo))
    );
} 