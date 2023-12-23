use async_rocket_example::{
    db::DB,
    routing::{create_todo, health_check, todo_list},
};
use rocket::routes;
use rocket_db_pools::Database;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .mount("/hc", routes![health_check])
        .mount("/todos", routes![todo_list, create_todo])
        .attach(DB::init())
        .launch()
        .await?;
    Ok(())
}
