use crate::todo::todo_ops::{self, TodoItem};


//=================================================================================
pub struct CreateTodoRequest {
    user_id: i32,
    description: String,
    set_dt: NaiveDate,
    color: String
}
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
pub struct DeleteTodoRequest {
    usr_id: i32,
    id_todo: i32,
}
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
pub struct ChangeTodoRequest {
    usr_id: i32,
    id_todo: i32,
    description: String,
    set_dt: NaiveDate,
    color: String
}
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
pub struct GetTodosRequest {

}
pub struct GetTodosResponse {
    success: bool,
    op_describe: String
}
//=================================================================================