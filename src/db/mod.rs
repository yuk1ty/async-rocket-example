use rocket_db_pools::{sqlx, Database};

pub mod models;

#[derive(Database)]
#[database("app")]
pub struct DB(sqlx::PgPool);
