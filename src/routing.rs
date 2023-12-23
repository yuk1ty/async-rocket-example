use rocket::{get, http::Status, serde::json::Json};

use crate::models::{TodoListResponse, TodoResponse};

#[get("/")]
pub fn health_check() -> Status {
    Status::Ok
}

#[get("/?<limit>&<done>")]
pub fn todo_list(limit: Option<usize>, done: Option<bool>) -> Json<TodoListResponse> {
    // A dummy data to represent fetching values from database.
    // This will be replaced with database accessing steps in the next part.
    let source = vec![
        TodoResponse {
            id: 1,
            title: "First todo".to_string(),
            description: "This is the first todo".to_string(),
            done: false,
        },
        TodoResponse {
            id: 2,
            title: "Second todo".to_string(),
            description: "This is the second todo".to_string(),
            done: false,
        },
        TodoResponse {
            id: 3,
            title: "Third todo".to_string(),
            description: "This is the third todo".to_string(),
            done: false,
        },
    ];
    // Here should be done within database.
    let filtered = source[..limit.unwrap_or(source.len())]
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    let filtered = filtered
        .into_iter()
        .filter(|todo| {
            if let Some(done) = done {
                todo.done == done
            } else {
                true
            }
        })
        .collect::<Vec<_>>();

    Json(TodoListResponse { items: filtered })
}
