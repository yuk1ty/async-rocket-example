use rocket::{
    outcome::Outcome,
    serde::{Deserialize, Serialize},
};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct CreateOrModifyTodoRequest {
    #[validate(length(min = 1, max = 256))]
    pub title: String,
    #[validate(length(max = 4096))]
    pub description: Option<String>,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for CreateOrModifyTodoRequest {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let todo = request
            .guard::<CreateOrModifyTodoRequest>()
            .await
            .expect("Invalid request body");

        if todo.validate().is_err() {
            return Outcome::Error((rocket::http::Status::BadRequest, ()));
        }

        Outcome::Success(todo)
    }
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TodoResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoListResponse {
    pub items: Vec<TodoResponse>,
}
