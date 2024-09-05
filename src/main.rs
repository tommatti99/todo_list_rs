#[macro_use] extern crate rocket;
use crate::todo::todo_api;

pub mod auth {
    pub mod jwt;
}
pub mod database {
    pub mod conec;
}
pub mod todo {
    pub mod todo_api;
    pub mod todo_models;
    pub mod todo_ops;
}

#[launch]
fn rocket() -> _ {
    rocket::build(
        .mount("/api", routes![todo_api::create_todo_api])
        .mount("/api", routes![todo_api::delete_todo_api])
        .mount("/api", routes![todo_api::change_todo_api])
        .mount("/api", routes![todo_api::get_todos_api])
        .mount("/api", routes![todo_api::get_todos_api])
    )
}