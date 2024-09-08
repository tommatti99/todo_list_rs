extern crate diesel;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use crate::auth::jwt; 
use crate::schema;
use crate::database::conec::start_connection;
use diesel::prelude::*;
use crate::utils::email;

use super::{login_ops, recover_ops};
use super::recover_ops::verify_recover_code_is_valid;

//=================================================================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginUserRequest {
    pub usr_id: i32,
    pub passw: String,
    pub token: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserResponse {
    pub status: bool,
    pub token: String,
}

impl LoginUserResponse {
    pub fn login_success(login_data: LoginUserRequest) -> Self {
        let new_token: String = jwt::get_jwt(login_data.usr_id);
        
        return LoginUserResponse {
            status: true,
            token: new_token
        }
    }
    pub fn login_passw_incorrect() -> Self {
        return LoginUserResponse {
            status: false,
            token: "error".to_string()
        }
    }

}
//=================================================================================

//=================================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverAccRequest {
    pub user_email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverAccResponse {
    pub recover_code_sent: bool,
}

impl RecoverAccResponse {
    pub fn send_recover_email(recover_acc_data: RecoverAccRequest) -> Self {
        let mut conec: PgConnection = start_connection();

        let usr_id: i32 = schema::users::dsl::users
            .select(schema::users::dsl::user_id)
            .filter(schema::users::dsl::email.eq(recover_acc_data.user_email.clone()))
            .first::<i32>(&mut conec).unwrap();


        let sent: bool = email::send_email(email::EmailMessage::recover_acc_email(recover_ops::gen_recover_code(usr_id), recover_acc_data.user_email));

        return RecoverAccResponse {
            recover_code_sent: sent
        };
    }
}
//=================================================================================

//=================================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverAccTokenValidationRequest {
    pub user_email: String,
    pub recover_code: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverAccTokenValidationResponse {
    pub status: bool,
    pub token: String
}

impl RecoverAccTokenValidationResponse {
    pub fn verify_recover_acc_code(recover_acc_code: RecoverAccTokenValidationRequest) -> Self {
        let mut conec: PgConnection = start_connection();

        let usr_id: i32 = schema::users::dsl::users
            .select(schema::users::dsl::user_id)
            .filter(schema::users::dsl::email.eq(recover_acc_code.user_email))
            .first::<i32>(&mut conec).unwrap();

        if verify_recover_code_is_valid(usr_id, recover_acc_code.recover_code) {


            let new_token: String = jwt::get_jwt(usr_id);
        
            return RecoverAccTokenValidationResponse {
                status: true,
                token: new_token
            }

        } else {
            return RecoverAccTokenValidationResponse {
                status: false,
                token: "Erro, código de recuperação invalido".to_string()
            }
        }
    }
}
//=================================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswRequest {
    pub user_email: String,
    pub token: String,
    pub new_passw: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswResponse {
    pub status: bool,
    pub description: String
}

impl ChangePasswResponse {
    pub fn execute_password_change(change_passw_data: ChangePasswRequest) -> Self {
        let mut conec: PgConnection = start_connection();

        let usr_id: i32 = schema::users::dsl::users
            .select(schema::users::dsl::user_id)
            .filter(schema::users::dsl::email.eq(change_passw_data.user_email.clone()))
            .first::<i32>(&mut conec).unwrap();

        if jwt::verify_jwt_token(usr_id, &change_passw_data.token.clone()) {
            login_ops::change_passw_by_token(usr_id, change_passw_data.user_email, change_passw_data.token, change_passw_data.new_passw);

            return ChangePasswResponse {
                status: true,
                description: "Senha alterada com sucesso".to_string()
            };

        } else {
            return ChangePasswResponse {
                status: false,
                description: "Senha incorreta".to_string()
            };
        }

    }
}
//=================================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct RenewTokenRequest {
    pub usr_id: i32,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenewTokenResponse {
    pub status: bool,
    pub new_token: String
} 
impl RenewTokenResponse {
    pub fn gen_new_token(login_data: RenewTokenRequest) -> Self {
        if jwt::verify_jwt_token(login_data.usr_id.clone(), &login_data.token.clone()) {
            let new_token = jwt::get_jwt(login_data.usr_id);

            return RenewTokenResponse {
                status: true,
                new_token: new_token
            };

        } else {
            return RenewTokenResponse {
                status: false,
                new_token: "Session expired".to_string()
            };
        }
    }
}