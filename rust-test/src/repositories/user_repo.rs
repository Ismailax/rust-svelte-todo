use crate::models::user::User;
use sqlx::{PgPool, Row};

pub async fn exists(pool: &PgPool, username: &str) -> bool {
    let row = sqlx::query("SELECT 1 FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
        .unwrap();

    row.is_some()
}

pub async fn create(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> Result<i32, sqlx::Error> {
    let row = sqlx::query("INSERT INTO users(username, password_hash) VALUES ($1,$2) RETURNING id")
        .bind(username)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

    Ok(row.get::<i32, _>("id"))
}

// ✅ ใช้ model ตรง ๆ
pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}

// ✅ ใช้ model + restrict fields (เลือกเฉพาะ field เพราะไม่ใช่ทุกที่ต้องการ password_hash)
#[derive(sqlx::FromRow)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
}

pub async fn find_user_by_id(pool: &PgPool, user_id: i32) -> Result<Option<UserInfo>, sqlx::Error> {
    sqlx::query_as::<_, UserInfo>("SELECT id, username FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}
