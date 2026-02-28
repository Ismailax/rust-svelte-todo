use actix_web::{HttpResponse, Responder, web};
use sqlx::PgPool;

pub async fn health_handler() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

pub async fn dbtest_handler(pool: web::Data<PgPool>) -> impl Responder {
    let res: Result<(i32,), sqlx::Error> =
        sqlx::query_as("SELECT 1").fetch_one(pool.get_ref()).await;

    match res {
        Ok((v,)) => HttpResponse::Ok().body(format!("db ok: {}", v)),
        Err(e) => {
            log::error!("dbtest error: {e}");
            HttpResponse::InternalServerError().body("db_error")
        }
    }
}
