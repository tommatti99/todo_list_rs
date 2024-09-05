use crate::todo::todo_ops::{self, TodoItem};
use rocket::serde::{Deserialize, Serialize}

//=================================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTodoRequest {
    user_id: i32,
    description: String,
    set_dt: NaiveDate,
    color: String
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTodoResponse {
    success: bool,
    op_describe: String
}
impl CreateTodoResponse {
    pub fn create(create_todo_data: CreateTodoRequest) -> Self {
        if TodoItem::new(create_todo_data.user_id, create_todo_data.description, create_todo_data.set_dt, create_todo_data.color) {
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
}
//=================================================================================




//=================================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteTodoRequest {
    usr_id: i32,
    id_todo: i32,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteTodoResponse {
    success: bool,
    op_describe: String
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
}
//=================================================================================




//=================================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct ChangeTodoRequest {
    usr_id: i32,
    id_todo: i32,
    description: String,
    set_dt: NaiveDate,
    color: String
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ChangeTodoResponse {
    success: bool,
    op_describe: String
}

impl ChangeTodoResponse {
    pub fn change(change_todo_data: ChangeTodoRequest) -> Self {
        if todo_ops::modify_todo(change_todo_data) {
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
}
//=================================================================================




//=================================================================================
#[derive(Deserialize, Serialize, Debug)]
pub struct GetTodosRequest {
    usr_id: i32,
    dt: NaiveDate
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GetTodosResponse {
    success: bool,
    op_describe: String
    todos: Vec<TodoItem>
}

impl GetTodosResponse {
    pub fn get_todos(get_todos_data: GetTodosRequest) -> Self {
        let user_todos: Vec<TodoItem> = todo_ops::get_month_todos(dt, usr_id, true);
        
        return Self {
            success: true,
            op_describe: "Operation ended with success".to_string(),
            todos: user_todos
        }
    } 
}
//=================================================================================