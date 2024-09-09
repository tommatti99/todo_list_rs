#[macro_use] extern crate rocket;
use crate::todo::todo_api;
use crate::auth::login_api;

pub mod auth {
    pub mod jwt;
    pub mod login_api;
    pub mod login_models;
    pub mod login_ops;
    pub mod recover_ops;
}
pub mod database {
    pub mod conec;
}
pub mod todo {
    pub mod todo_api;
    pub mod todo_models;
    pub mod todo_ops;
}
pub mod utils {
    pub mod email;
}
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/app/", routes![todo_api::create_todo_api])
        .mount("/api/app/", routes![todo_api::delete_todo_api])
        .mount("/api/app/", routes![todo_api::change_todo_api])
        .mount("/api/app/", routes![todo_api::get_todos_api])
        
        .mount("/api/auth/", routes![login_api::renew_token_api])
        .mount("/api/auth/", routes![login_api::change_passw_api])
        .mount("/api/auth/", routes![login_api::user_acc_recover_validate_code_api])
        .mount("/api/auth/", routes![login_api::user_acc_recover_call_api])
        .mount("/api/auth/", routes![login_api::user_login_api])
        .mount("/api/auth/", routes![login_api::create_account_api])
}
 
 
 
 
