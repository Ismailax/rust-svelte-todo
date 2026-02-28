use serde::{Deserialize, Serialize};

// Request DTOs

#[derive(Deserialize)]
pub struct RegisterReq {
    pub username: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

// Response DTOs

#[derive(Serialize, Clone)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub user: UserResponse,
    pub token: String,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub message: String,
}
