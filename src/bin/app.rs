use async_rocket_example::routing::{health_check, todo_list};
use rocket::routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .mount("/hc", routes![health_check])
        .mount("/todos", routes![todo_list])
        .launch()
        .await?;
    Ok(())
}
