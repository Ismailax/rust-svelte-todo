use actix_web::web;

use crate::handlers::{
    auth_handler::{login_handler, logout_handler, me_handler, register_handler},
    system_handler::{dbtest_handler, health_handler},
    todo_handler::{create_todo, delete_todo, list_todos, update_todo},
};

pub fn api_scope() -> actix_web::Scope {
    web::scope("")
        // ---- System ----
        .route("/health", web::get().to(health_handler))
        .route("/dbtest", web::get().to(dbtest_handler))
        // ---- Auth ----
        .route("/register", web::post().to(register_handler))
        .route("/login", web::post().to(login_handler))
        .route("/logout", web::post().to(logout_handler))
        .route("/me", web::get().to(me_handler))
        // ---- Todos ----
        .route("/todos", web::get().to(list_todos))
        .route("/todos", web::post().to(create_todo))
        .route("/todos/{id}", web::put().to(update_todo))
        .route("/todos/{id}", web::delete().to(delete_todo))
}
