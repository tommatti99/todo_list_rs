const ROOT: &str= "";
//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "token": String
//              "user_id": i32,
//              "title": String
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
const CREATE_TODO_API: &str= "api/app/new_todo";
//=================================================================================




//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "token": String
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
const  DELETE_TODO_API: &str= "api/app/del_todo";
//=================================================================================




//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "token": String
//              "usr_id": i32,
//              "id_todo": i32,
//              "title": String
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
const CHANGE_TODO_API: &str= "api/app/modify_todo";
//=================================================================================




//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "token": String
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
const GET_TODOS_API: &str= "api/app/get_todos";
//=================================================================================



//=================================================================================
//  REQUEST:
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "user_email": "admin", 
//              "passw": "admin"
//          }
//    
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "status": bool,
//              "token": String 
//          }
//
const USER_LOGIN_API: &str= "api/auth/authenticate";
//=================================================================================




//=================================================================================
// REQUEST
//      Header: 
//          Content-Type: application/json
//      Body: 
//          {
//              "user_email": String,
//          }
//
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "recover_code_sent": bool,
//          }
//
const USER_ACC_RECOVER_CALL_API: &str= "api/auth/recover";
//=================================================================================




//=================================================================================
// REQUEST
//      Header: Content-Type: application/json
//      Body: 
//          {
//              "user_email": String,
//              "recover_code": i16
//          }
//
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "status": bool,
//              "token": String
//          }
//
const USER_ACCRECOVER_VALIDATE_CODE_API: &str= "api/auth/recover_code";
//=================================================================================




//=================================================================================
// REQUEST
//      Header: Content-Type: application/json
//      Body: 
//          {
//              "user_email": String,
//              "token": String
//          }
//
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "status": bool
//              "description": String
//          }
//
const CHANGE_PASSW_API: &str= "api/auth/change_passw";
//=================================================================================




//=================================================================================
// REQUEST
//      Header: Content-Type: application/json
//      Body: 
//          {
//              "user_email": String,
//              "token": String
//          }
//
// RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "status": bool
//              "new_token": String
//          }
//
const RENEW_TOKEN_API: &str= "api/auth/new_token";

const GET_COLORS: &str = "";