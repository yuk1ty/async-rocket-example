use rocket::{get, http::Status, post, response::status::Created, serde::json::Json};
use rocket_db_pools::Connection;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::DB,
    errors::Errors,
    models::{CreateOrModifyTodoRequest, TodoListResponse, TodoResponse},
};

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
            id: Uuid::new_v4(),
            title: "First todo".to_string(),
            description: Some("This is the first todo".to_string()),
            done: false,
        },
        TodoResponse {
            id: Uuid::new_v4(),
            title: "Second todo".to_string(),
            description: Some("This is the second todo".to_string()),
            done: false,
        },
        TodoResponse {
            id: Uuid::new_v4(),
            title: "Third todo".to_string(),
            description: Some("This is the third todo".to_string()),
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

#[post("/", format = "json", data = "<todo>")]
pub async fn create_todo(
    todo: Json<CreateOrModifyTodoRequest>,
    mut db: Connection<DB>,
) -> crate::errors::Result<Created<Json<TodoResponse>>> {
    todo.validate().map_err(Errors::ValidationError)?;
    let res = sqlx::query!(
        r#"
        INSERT INTO todos (title, description, done)
        VALUES ($1, $2, false)
        RETURNING id, title, description, done
        "#,
        todo.title,
        todo.description
    )
    .fetch_one(&mut **db)
    .await
    .map(|r| TodoResponse {
        id: r.id,
        title: r.title,
        description: r.description,
        done: r.done,
    })
    .map_err(Errors::SqlxError)?;
    Ok(Created::new("/").body(Json(res)))
}
