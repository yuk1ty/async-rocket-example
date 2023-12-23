use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct TodoRow {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub creation_date: NaiveDateTime,
    pub modified_date: Option<NaiveDateTime>,
}
