use rocket::serde::json::Json;
use crate::auth::login_ops;
use crate::auth::login_models::{LoginUserRequest, LoginUserResponse,
                                RecoverAccRequest, RecoverAccResponse, 
                                RecoverAccTokenValidationRequest, RecoverAccTokenValidationResponse,
                                ChangePasswRequest, ChangePasswResponse, 
                                RenewTokenRequest, RenewTokenResponse};
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

#[post("/authenticate", format = "json", data = "<login_data_json>")]
pub fn user_login_api(login_data_json: Json<LoginUserRequest>) -> rocket::serde::json::Json<LoginUserResponse>  {

    let login_data: LoginUserRequest = login_data_json.into_inner();

    if login_ops::validate_hash(login_data.clone()) {
        return Json(LoginUserResponse::login_success(login_data));

    } else {
        return Json(LoginUserResponse::login_passw_incorrect());
    }
}
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
#[post("/recover", format = "json", data = "<recover_acc_data_json>")]
pub fn user_acc_recover_call_api(recover_acc_data_json: Json<RecoverAccRequest>) -> Json<RecoverAccResponse> {
    let recover_acc_data: RecoverAccRequest = recover_acc_data_json.into_inner();

    return Json(RecoverAccResponse::send_recover_email(recover_acc_data))
}
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
#[post("/recover_code", format = "json", data = "<recover_acc_code_json>")]
pub fn user_acc_recover_validate_code_api(recover_acc_code_json: Json<RecoverAccTokenValidationRequest>) -> Json<RecoverAccTokenValidationResponse> {
    let recover_acc_code: RecoverAccTokenValidationRequest = recover_acc_code_json.into_inner();

    return Json(RecoverAccTokenValidationResponse::verify_recover_acc_code(recover_acc_code));    
}
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
#[post("/change_passw", format = "json", data = "<change_passw_data_json>")]
pub fn change_passw_api(change_passw_data_json: Json<ChangePasswRequest>) -> Json<ChangePasswResponse> {
    let change_passw_data: ChangePasswRequest = change_passw_data_json.into_inner();

    return Json(ChangePasswResponse::execute_password_change(change_passw_data));
}
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
#[post("/new_token", format = "json", data = "<new_token_data_json>")]
pub fn renew_token_api(new_token_data_json: RenewTokenRequest) -> Json<RenewTokenResponse> {

    let new_token_data: RenewTokenRequest = new_token_data_json.into_inner();

    return Json(RenewTokenResponse::gen_new_token(new_token_data));
}