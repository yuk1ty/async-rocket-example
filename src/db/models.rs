use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct TodoRow {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub creation_date: NaiveDateTime,
    pub modified_date: Option<NaiveDateTime>,
}
