mod config;
mod db;
mod dto;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use crate::config::AppConfig;
use crate::db::get_db_pool;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use env_logger::Env;
use sqlx::Pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg: AppConfig = config::load_env();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let pool: Pool<sqlx::Postgres> = get_db_pool().await;

    // clone ไว้ใช้ทั้งใน closure และที่ .bind ด้านล่าง
    let cfg_for_server = cfg.clone();

    HttpServer::new(move || {
        // clone อีกชั้นสำหรับใช้ในแต่ละ worker
        let cfg_in_app = cfg_for_server.clone();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(cfg_in_app.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin(cfg_in_app.frontend_url.as_str())
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        header::CONTENT_TYPE,
                        header::ACCEPT,
                        header::AUTHORIZATION,
                    ])
                    .supports_credentials(),
            )
            .wrap(Logger::default())
            .service(routes::api_scope())
    })
    .bind((cfg.host.as_str(), cfg.port))?
    .run()
    .await
}
