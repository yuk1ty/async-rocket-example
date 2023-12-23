use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateOrModifyTodoRequest {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TodoResponse {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoListResponse {
    pub items: Vec<TodoResponse>,
}
