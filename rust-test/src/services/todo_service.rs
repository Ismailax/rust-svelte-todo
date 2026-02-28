use crate::dto::todo::{CreateTodoReq, TodoResponse, UpdateTodoReq};
use crate::models::todo::Todo;
use crate::repositories::todo_repo;
use sqlx::PgPool;

pub async fn list(pool: &PgPool, user_id: i32) -> Result<Vec<TodoResponse>, String> {
    let rows: Vec<Todo> = todo_repo::get_by_user(pool, user_id)
        .await
        .map_err(|_| "db_error".to_string())?;

    Ok(rows
        .into_iter()
        .map(|t| TodoResponse {
            id: t.id,
            title: t.title,
            completed: t.completed,
        })
        .collect())
}

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    req: CreateTodoReq,
) -> Result<TodoResponse, String> {
    let row: Todo = todo_repo::create(pool, user_id, &req.title)
        .await
        .map_err(|_| "db_error".to_string())?;

    Ok(TodoResponse {
        id: row.id,
        title: row.title,
        completed: row.completed,
    })
}

pub async fn update(
    pool: &PgPool,
    user_id: i32,
    id: i32,
    req: UpdateTodoReq,
) -> Result<TodoResponse, String> {
    let row: Todo = todo_repo::update(pool, id, user_id, req.title, req.completed)
        .await
        .map_err(|_| "db_error".to_string())?;

    Ok(TodoResponse {
        id: row.id,
        title: row.title,
        completed: row.completed,
    })
}

pub async fn delete(pool: &PgPool, user_id: i32, id: i32) -> Result<(), String> {
    let affected: u64 = todo_repo::delete(pool, id, user_id)
        .await
        .map_err(|_| "db_error".to_string())?;

    if affected == 0 {
        return Err("not_found".to_string());
    }
    Ok(())
}
