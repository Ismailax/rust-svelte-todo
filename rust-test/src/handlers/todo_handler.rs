use crate::{
    config::AppConfig,
    dto::todo::{CreateTodoReq, UpdateTodoReq},
    services::todo_service,
    utils::auth::extract_token,
    utils::jwt::verify_token,
};
use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

fn get_user_id(req: &actix_web::HttpRequest, cfg: &AppConfig) -> Result<i32, HttpResponse> {
    let token = extract_token(req).ok_or_else(|| HttpResponse::Unauthorized().finish())?;

    let claims = verify_token(&token, cfg).map_err(|_| HttpResponse::Unauthorized().finish())?;

    claims
        .sub
        .parse::<i32>()
        .map_err(|_| HttpResponse::Unauthorized().finish())
}

pub async fn list_todos(
    req: actix_web::HttpRequest,
    pool: web::Data<PgPool>,
    cfg: web::Data<AppConfig>,
) -> impl Responder {
    let uid = match get_user_id(&req, &cfg) {
        Ok(u) => u,
        Err(e) => return e,
    };

    match todo_service::list(pool.get_ref(), uid).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_todo(
    req: actix_web::HttpRequest,
    pool: web::Data<PgPool>,
    cfg: web::Data<AppConfig>,
    payload: web::Json<CreateTodoReq>,
) -> impl Responder {
    let uid = match get_user_id(&req, &cfg) {
        Ok(u) => u,
        Err(e) => return e,
    };

    match todo_service::create(pool.get_ref(), uid, payload.into_inner()).await {
        Ok(todo) => HttpResponse::Created().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_todo(
    req: actix_web::HttpRequest,
    pool: web::Data<PgPool>,
    cfg: web::Data<AppConfig>,
    path: web::Path<i32>,
    payload: web::Json<UpdateTodoReq>,
) -> impl Responder {
    let id = path.into_inner();
    let uid = match get_user_id(&req, &cfg) {
        Ok(u) => u,
        Err(e) => return e,
    };

    match todo_service::update(pool.get_ref(), uid, id, payload.into_inner()).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_todo(
    req: actix_web::HttpRequest,
    pool: web::Data<PgPool>,
    cfg: web::Data<AppConfig>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    let uid = match get_user_id(&req, &cfg) {
        Ok(u) => u,
        Err(e) => return e,
    };

    match todo_service::delete(pool.get_ref(), uid, id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
