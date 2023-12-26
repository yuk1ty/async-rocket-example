use rocket::{get, http::Status, post, put, response::status::Created, serde::json::Json};
use rocket_db_pools::Connection;
use rocket_errors::anyhow;
use validator::Validate;

use crate::{
    db::{models::TodoRow, DB},
    errors::Errors,
    models::{CreateOrModifyTodoRequest, TodoListResponse, TodoResponse},
    Uuid,
};

#[get("/")]
pub fn health_check() -> Status {
    Status::Ok
}

#[get("/?<limit>&<done>")]
pub async fn todo_list(
    limit: Option<usize>,
    done: Option<bool>,
    mut db: Connection<DB>,
) -> anyhow::Result<Json<TodoListResponse>> {
    let filtered: Vec<TodoRow> = match (limit, done) {
        (Some(limit), Some(done)) => sqlx::query_as!(
                TodoRow,
                "SELECT id, title, description, done, creation_date, modified_date FROM todos WHERE done = $1 LIMIT $2",
                done,
                limit as i64
            )
            .fetch_all(&mut **db)
            .await
            .map_err(Errors::SqlxError)?,
        (Some(limit), None) => sqlx::query_as!(
                TodoRow,
                "SELECT id, title, description, done, creation_date, modified_date FROM todos LIMIT $1",
                limit as i64
            )
            .fetch_all(&mut **db)
            .await
            .map_err(Errors::SqlxError)?,
        (None, Some(done)) => sqlx::query_as!(
                TodoRow,
                "SELECT id, title, description, done, creation_date, modified_date FROM todos WHERE done = $1",
                done
            )
            .fetch_all(&mut **db)
            .await
            .map_err(Errors::SqlxError)?,
        (None, None) => sqlx::query_as!(TodoRow, "SELECT id, title, description, done, creation_date, modified_date FROM todos")
            .fetch_all(&mut **db)
            .await
            .map_err(Errors::SqlxError)?,
    };

    let res = filtered
        .into_iter()
        .map(|r| TodoResponse {
            id: r.id,
            title: r.title,
            description: r.description,
            done: r.done,
        })
        .collect::<Vec<_>>();

    Ok(Json(TodoListResponse { items: res }))
}

#[post("/", format = "json", data = "<todo>")]
pub async fn create_todo(
    todo: Json<CreateOrModifyTodoRequest>,
    mut db: Connection<DB>,
) -> anyhow::Result<Created<Json<TodoResponse>>> {
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

#[put("/<id>/done")]
pub async fn make_todo_done(
    id: Uuid,
    mut db: Connection<DB>,
) -> anyhow::Result<Json<TodoResponse>> {
    let res = sqlx::query!(
        r#"
        UPDATE todos
        SET done = true 
        WHERE id = $1
        RETURNING id, title, description, done
        "#,
        id.0
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
    Ok(Json(res))
}
