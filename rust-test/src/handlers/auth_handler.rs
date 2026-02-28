use crate::config::AppConfig;
use crate::dto::auth::{AuthResponse, LoginReq, LogoutResponse, RegisterReq, UserResponse};
use crate::repositories::user_repo;
use crate::services::auth_service;
use crate::utils::auth::extract_token;
use crate::utils::jwt::{build_auth_cookie, clear_auth_cookie, create_access_token, verify_token};

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use sqlx::PgPool;

pub async fn register_handler(
    pool: web::Data<PgPool>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<RegisterReq>,
) -> impl Responder {
    let body = payload.into_inner();

    match auth_service::register(
        pool.get_ref(),
        &body.username,
        &body.password,
        &body.password_confirmation,
    )
    .await
    {
        Ok(user) => match create_access_token(user.id, &cfg) {
            Ok(token) => {
                let cookie = build_auth_cookie(&token, &cfg);
                HttpResponse::Created().cookie(cookie).json(AuthResponse {
                    message: "registered".into(),
                    user,
                    token,
                })
            }
            Err(e) => HttpResponse::InternalServerError().body(e),
        },
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn login_handler(
    pool: web::Data<PgPool>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<LoginReq>,
) -> impl Responder {
    let body = payload.into_inner();

    match auth_service::login(pool.get_ref(), &body.username, &body.password).await {
        Ok(user) => match create_access_token(user.id, &cfg) {
            Ok(token) => {
                let cookie = build_auth_cookie(&token, &cfg);
                HttpResponse::Ok().cookie(cookie).json(AuthResponse {
                    message: "logged_in".into(),
                    user,
                    token,
                })
            }
            Err(e) => HttpResponse::InternalServerError().body(e),
        },
        Err(e) if e == "invalid_credentials" => HttpResponse::Unauthorized().body(e),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn me_handler(
    req: HttpRequest,
    cfg: web::Data<AppConfig>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let Some(token) = extract_token(&req) else {
        return HttpResponse::Unauthorized().body("no_token");
    };

    let claims = match verify_token(&token, &cfg) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().body("invalid_token"),
    };

    let uid = match claims.sub.parse::<i32>() {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().body("invalid_user_id"),
    };

    match user_repo::find_user_by_id(pool.get_ref(), uid).await {
        Ok(Some(u)) => HttpResponse::Ok().json(UserResponse {
            id: u.id,
            username: u.username,
        }),
        Ok(None) => HttpResponse::Unauthorized().body("user_not_found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn logout_handler(cfg: web::Data<AppConfig>) -> impl Responder {
    HttpResponse::Ok()
        .cookie(clear_auth_cookie(&cfg))
        .json(LogoutResponse {
            message: "logged_out".into(),
        })
}
