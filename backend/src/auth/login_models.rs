extern crate diesel;
use serde::{Deserialize, Serialize};
use crate::user::profile::profile_ops;
use crate::user::login::jwt;
use crate::utils::email::email_models::EmailMessage;
use crate::utils::email::email;
use crate::CustomErrors;
use crate::utils::fast_ops_diesel;
use crate::user::login::recover_ops::verify_recover_code_is_valid;

//=================================================================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginUserRequest {
    pub user_email: String,
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
        let new_token: String = jwt::get_jwt(fast_ops_diesel::user_id_by_email(login_data.user_email));
        
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
    pub user_email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverAccResponse {
    pub recover_code_sent: bool,
}

impl RecoverAccResponse {
    pub fn send_recover_email(recover_acc_data: RecoverAccRequest) -> Self {
        let usr_id: i16 = fast_ops_diesel::user_id_by_email(recover_acc_data.user_email);
        let sent: bool = email::send_email(EmailMessage::recover_acc(usr_id));

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
    pub recover_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverAccTokenValidationResponse {
    pub status: bool,
    pub token: String
}

impl RecoverAccTokenValidationResponse {
    pub fn verify_recover_acc_code(recover_acc_code: RecoverAccTokenValidationRequest) -> Self {
        if verify_recover_code_is_valid(recover_acc_code.user_email, recover_acc_code.recover_code) {
            let new_token: String = jwt::get_jwt(fast_ops_diesel::user_id_by_email(recover_acc_code.user_email));
        
            return RecoverAccTokenValidationResponse {
                status: true,
                token: new_token
            }

        } else {
            return RecoverAccTokenValidationResponse {
                status: false,
                token: CustomErrors::RecoverCodeWrong.describe().to_string()
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
    pub description: bool
}

impl ChangePasswResponse {
    pub fn execute_password_change(change_passw_data: ChangePasswRequest) -> Self {
        if jwt::verify_jwt(&change_passw_data.token.clone()) {
           profile_ops::change_passw_by_token(change_passw_data);


        } else {
            return ChangePasswResponse {
                status: false,
                description: CustomErrors::SessionExpired.describe()
            };
        }

    }
}
//=================================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct RenewTokenRequest {
    pub user_email: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenewTokenResponse {
    pub status: bool,
    pub new_token: String
} 
impl RenewTokenResponse {
    pub fn gen_new_token(login_data: RenewTokenRequest) -> Self {
        if jwt::verify_jwt(&login_data.token.clone()) {
            let new_token = jwt::get_jwt(fast_ops_diesel::user_id_by_email(login_data.user_email));

            return RenewTokenResponse {
                status: true,
                new_token: new_token
            };

        } else {
            return RenewTokenResponse {
                status: false,
                new_token: CustomErrors::SessionExpired.describe().to_string()
            };
        }
    }
}