use crate::dto::auth::UserResponse;
use crate::repositories::user_repo;
use sqlx::PgPool;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub async fn register(
    pool: &PgPool,
    username: &str,
    password: &str,
    password_confirmation: &str,
) -> Result<UserResponse, String> {
    if !(1..=30).contains(&username.len()) {
        return Err("invalid_username_length".into());
    }
    if password != password_confirmation {
        return Err("passwords_do_not_match".into());
    }
    if password.len() < 8 {
        return Err("password_too_short".into());
    }
    if user_repo::exists(pool, username).await {
        return Err("username_already_exists".into());
    }

    let hashed = hash_password(password)?;
    let user_id = user_repo::create(pool, username, &hashed)
        .await
        .map_err(|_| "database_error".to_string())?;

    Ok(UserResponse {
        id: user_id,
        username: username.to_string(),
    })
}

pub async fn login(pool: &PgPool, username: &str, password: &str) -> Result<UserResponse, String> {
    if let Some(user) = user_repo::find_by_username(pool, username.trim())
        .await
        .map_err(|_| "database_error".to_string())?
    {
        verify_password(password, &user.password_hash)
            .map_err(|_| "invalid_credentials".to_string())?;

        return Ok(UserResponse {
            id: user.id,
            username: user.username.clone(),
        });
    }

    Err("invalid_credentials".into())
}

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| "hash_error".to_string())
        .map(|hash| hash.to_string())
}

fn verify_password(password: &str, stored_hash: &str) -> Result<(), String> {
    let parsed_hash = PasswordHash::new(stored_hash).map_err(|_| "invalid_hash".to_string())?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| "invalid_password".to_string())
}
