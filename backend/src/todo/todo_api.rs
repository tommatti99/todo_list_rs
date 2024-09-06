use rocket::serde::json::Json;
use crate::todo::todo_ops::is_todo_owner;
use crate::todo::todo_models::{CreateTodoRequest, CreateTodoResponse,
                         DeleteTodoRequest, DeleteTodoResponse,
                         ChangeTodoRequest, ChangeTodoResponse,
                         GetTodosRequest,   GetTodosResponse};

//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "user_id": i32,
//              "description": String,
//              "set_dt": NaiveDate,
//              "color": String
//          }
//    
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "success": bool,
//              "op_describe": String
//          }
//
#[post("/new_todo", format = "json", data = "<create_todo_data_json>")]
pub fn create_todo_api(create_todo_data_json: Json<CreateTodoRequest>) -> Json<CreateTodoResponse> {
    let create_todo_data: CreateTodoRequest = create_todo_data_json.into_inner();

    return Json(CreateTodoResponse::create(create_todo_data));
}
//=================================================================================




//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "usr_id": i32,
//              "id_todo": i32
//          }
//    
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "success": bool,
//              "op_describe": String
//          }
//
#[post("/del_todo", format = "json", data = "<delete_todo_data_json>")]
pub fn delete_todo_api(delete_todo_data_json: Json<DeleteTodoRequest>) -> Json<DeleteTodoResponse> {
    let delete_todo_data: DeleteTodoRequest = delete_todo_data_json.into_inner();

    if is_todo_owner(delete_todo_data.usr_id, delete_todo_data.id_todo) {
        return Json(DeleteTodoResponse::deleted(delete_todo_data));
    } else {
        return Json(DeleteTodoResponse::rejected());
    }
}
//=================================================================================




//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "usr_id": i32,
//              "id_todo": i32,
//              "description": String,
//              "set_dt": NaiveDate,
//              "color": String
//          }
//    
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "success": bool,
//              "op_describe": String
//          }
//
#[post("/modify_todo", format = "json", data = "<change_todo_data_json>")]
pub fn change_todo_api(change_todo_data_json: Json<ChangeTodoRequest>) -> Json<ChangeTodoResponse> {
    let change_todo_data: ChangeTodoRequest = change_todo_data_json.into_inner();

    if is_todo_owner(change_todo_data.usr_id, change_todo_data.id_todo) {
        return Json(ChangeTodoResponse::change(change_todo_data));
    } else {
        return Json(ChangeTodoResponse::rejected());
    }
}
//=================================================================================




//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "usr_id": i32,
//              "dt": NaiveDate
//          }
//    
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "success": bool,
//              "op_describe": String,
//              "todos": Vec<TodoItem>
//          }
//
#[post("/get_todos", format = "json", data = "<get_todos_data_json>")]
pub fn get_todos_api(get_todos_data_json: Json<GetTodosRequest>) -> Json<GetTodosResponse> {
    let get_todos_data: GetTodosRequest = get_todos_data_json.into_inner();

    return Json(GetTodosResponse::get_todos(get_todos_data));
}
//=================================================================================