use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTodoReq {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoReq {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
