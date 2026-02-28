use crate::models::todo::Todo;
use sqlx::{PgPool, Result};

pub async fn get_by_user(pool: &PgPool, user_id: i32) -> Result<Vec<Todo>> {
    sqlx::query_as::<_, Todo>(
        "
            SELECT id, user_id, title, completed, created_at, updated_at
            FROM todos
            WHERE user_id = $1
            ORDER BY id ASC
            ",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn create(pool: &PgPool, user_id: i32, title: &str) -> Result<Todo> {
    sqlx::query_as::<_, Todo>("INSERT INTO todos (user_id, title) VALUES ($1, $2) RETURNING *")
        .bind(user_id)
        .bind(title)
        .fetch_one(pool)
        .await
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    user_id: i32,
    title: Option<String>,
    completed: Option<bool>,
) -> Result<Todo> {
    sqlx::query_as::<_, Todo>(
        "UPDATE todos
         SET title = COALESCE($1, title),
             completed = COALESCE($2, completed),
             updated_at = NOW()
         WHERE id = $3 AND user_id = $4
         RETURNING *",
    )
    .bind(title)
    .bind(completed)
    .bind(id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: i32, user_id: i32) -> Result<u64> {
    let result = sqlx::query("DELETE FROM todos WHERE id=$1 AND user_id=$2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
