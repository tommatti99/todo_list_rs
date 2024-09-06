use crate::todo::todo_ops::{self, TodoItem};
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDate;

//=================================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTodoRequest {
    pub token: String,
    pub user_id: i32,
    pub description: String,
    pub set_dt: NaiveDate,
    pub color: String
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTodoResponse {
    pub success: bool,
    pub op_describe: String
}
impl CreateTodoResponse {
    pub fn create(create_todo_data: CreateTodoRequest) -> Self {
        if TodoItem::new(create_todo_data.user_id, create_todo_data.title, create_todo_data.description, create_todo_data.set_dt, create_todo_data.color) {
            return Self {
                success: true,
                op_describe: "Operation ended with success".to_string()
            }
        } else {    
            return Self {
                success: false,
                op_describe: "Operation not concluded. Internal Error".to_string()
            }
        }
    }
    pub fn session_expired() -> Self {
        Self {
            success: true,
            op_describe: "Session expired".to_string()
        }
    }
}
//=================================================================================




//=================================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteTodoRequest {
    pub token: String,
    pub usr_id: i32,
    pub id_todo: i32,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteTodoResponse {
    pub success: bool,
    pub op_describe: String
}
impl DeleteTodoResponse {
    pub fn deleted(delete_todo_data: DeleteTodoRequest) -> Self {
        if todo_ops::delete_todo(delete_todo_data.id_todo) {
            return Self {
                success: true,
                op_describe: "Operation ended with success".to_string()
            }
        } else {
            return Self {
                success: false,
                op_describe: "Operation not concluded. Internal Error".to_string()
            }
        }
    }

    pub fn rejected() -> Self {
        return Self {
            success: false,
            op_describe: "Operation not permitted. Not the todo owner".to_string()
        }
    }

    pub fn session_expired() -> Self {
        Self {
            success: true,
            op_describe: "Session expired".to_string()
        }
    }
}
//=================================================================================




//=================================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct ChangeTodoRequest {
    pub token: String,
    pub usr_id: i32,
    pub id_todo: i32,
    pub title: String,
    pub description: String,
    pub set_dt: NaiveDate,
    pub color: String
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ChangeTodoResponse {
    pub success: bool,
    pub op_describe: String
}

impl ChangeTodoResponse {
    pub fn change(change_todo_data: ChangeTodoRequest) -> Self {
        if todo_ops::modify_todo(change_todo_data.id_todo, change_todo_data.title, change_todo_data.description, change_todo_data.set_dt, change_todo_data.color) {
            return Self {
                success: true,
                op_describe: "Operation ended with success".to_string()
            }
        } else {
            return Self {
                success: false,
                op_describe: "Operation not concluded. Internal Error".to_string()
            }
        }
    }

    pub fn rejected() -> Self {
        return Self {
            success: false,
            op_describe: "Operation not permitted. Not the todo owner".to_string()
        }
    }

    pub fn session_expired() -> Self {
        Self {
            success: true,
            op_describe: "Session expired".to_string()
        }
    }
}
//=================================================================================




//=================================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct GetTodosRequest {
    pub token: String,
    pub usr_id: i32,
    pub dt: NaiveDate
}
#[derive(Deserialize, Serialize)]
pub struct GetTodosResponse {
    pub success: bool,
    pub op_describe: String,
    pub todos: Vec<TodoItem>
}

impl GetTodosResponse {
    pub fn get_todos(get_todos_data: GetTodosRequest) -> Self {
        let user_todos: Vec<TodoItem> = TodoItem::get_month_todos(get_todos_data.dt, get_todos_data.usr_id, true);
        
        return Self {
            success: true,
            op_describe: "Operation ended with success".to_string(),
            todos: user_todos
        }
    } 

    pub fn session_expired() -> Self {
        Self {
            success: true,
            op_describe: "Session expired".to_string()
        }
    }
}
//=================================================================================