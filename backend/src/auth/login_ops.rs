use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},Argon2, PasswordHash};
use diesel::PgConnection;
use diesel::prelude::*;
use crate::errors::CustomErrors; 
use crate::schema::users;
use crate::database::conec::start_connection;
use crate::user::login::login_models::LoginUserRequest;
use crate::schema;
use crate::schema::users::salt_hash_bytes;

fn new_salt() -> SaltString {
    return SaltString::generate(&mut OsRng);
}

pub fn new_hash(passw: &String) -> Result<String, CustomErrors> {
    let salt: SaltString = new_salt();
    match Argon2::default().hash_password(passw.as_bytes(), &salt) {
        Ok(hash) => {
            return Ok(hash.to_string())
        },
        Err(_) => {
            return Err(CustomErrors::PasswordNotValid);
        }
    }
}

pub fn validate_hash(login_data: LoginUserRequest) -> bool {
    let argon2id = Argon2::default();
    let user_hash_on_db = get_salt_hash_on_db(login_data.user_email).unwrap();
    
    match PasswordHash::new(user_hash_on_db.as_str()) {
        Ok(password_hash) => {
            match PasswordVerifier::verify_password(&argon2id, &login_data.passw.as_bytes(), &password_hash) {
                Ok(_) => {       
                    return true;
                },
                Err(_) => {
                    return false;
                }
            }
        },
        Err(_) => {
            return false;
        }
    }
}

pub fn get_salt_hash_on_db(usr_email: String) -> Result<String, CustomErrors> {
    let mut conec: PgConnection = start_connection();
    
    match schema::users::dsl::users
            .filter(schema::users::dsl::email.eq(usr_email))
            .select(salt_hash_bytes)
            .first::<Vec<u8>>(&mut conec) {
                Ok(hash_vec) => {
                    match std::str::from_utf8(&hash_vec.as_slice()) {
                        Ok(hash_str) => {
                            return Ok(hash_str.to_string());
                        },
                        Err(_) => {
                            return Err(CustomErrors::UserVerifierError);
                        }
                    }
                },
                Err(_) => {
                    return Err(CustomErrors::UnavailableDataBase);
                }
    };
}

pub fn verify_user_exists(usr_email: String) -> Result<bool, CustomErrors> {
    let mut conn: PgConnection = start_connection();
    match 
        diesel::select(diesel::dsl::exists(
        users::table.filter(users::email.eq(usr_email))
    ))
    .get_result::<bool>(& mut conn) {
        Ok(usr_found) => {
            return Ok(usr_found);
        },
        Err(_) => {
            return Err(CustomErrors::UserVerifierError);
        }
    }
}

#[cfg(test)]
mod test_login_ops {

    use crate::user::profile::profile_ops::create_new_user;
    use crate::errors::CustomMessages;
    use super::*;
    
    #[test]
    fn creating_a_new_user() {
        let created = create_new_user("jango".to_string(), "jangao@bol.com".to_string(), &"123".to_string());
        assert_eq!(created.unwrap().describe(), CustomMessages::UserCreatedWithSuccess.describe());
        let _creating_again = create_new_user("jango".to_string(), "jangao@bol.com".to_string(), &"123".to_string());
        assert_eq!(_creating_again.unwrap().describe(), CustomErrors::UserAlreadyExists.describe());
    }

    #[test]
    fn salt_len() {
        assert_eq!(new_salt().len(), 22);
        assert_ne!(new_salt().len(), 21);
    }

    #[test]
    fn rand_salt() {
        let salt_1: SaltString = SaltString::generate(&mut OsRng);
        let salt_2: SaltString = SaltString::generate(&mut OsRng);
        assert_ne!(salt_1, salt_2);
    }
    #[test]
    fn hash_always_ne() {
        let testpassw_1: String = String::from("123");
        let testpassw_2: String = String::from("123");

        let hash_1 = new_hash(&testpassw_1).expect(CustomErrors::PasswordNotValid.describe());
        let hash_2 = new_hash(&testpassw_2).expect(CustomErrors::PasswordNotValid.describe());
        assert_ne!(hash_1, hash_2);
    }

    #[test]
    fn verify_user_exists_test() {
        assert_eq!(verify_user_exists("jangao@bol.com".to_string()).unwrap() , true);
        assert_eq!(verify_user_exists("notcreateduser@bol.com".to_string()).unwrap() , false);
        assert_ne!(verify_user_exists("jangao@bol.com".to_string()).unwrap() , false);
        assert_ne!(verify_user_exists("notcreateduser@bol.com".to_string()).unwrap() , true);
    }

    #[test]
    fn get_data_from_db() {
        let user_salt_hash = get_salt_hash_on_db(String::from("jangao@bol.com")).unwrap();
        let new_hash = new_hash(&"123".to_string()).unwrap();
        assert_ne!(user_salt_hash, new_hash);
        assert_eq!(user_salt_hash.len(), 22);
    }

    #[test]
    fn validate_hash_test() {

        let user_data_correct: LoginUserRequest = 
        LoginUserRequest {
                user_email: "jangao@bol.com".to_string(),
                passw: "123".to_string(),
                token: None 
        };
        let user_data_incorrect: LoginUserRequest = 
            LoginUserRequest {
                user_email: "jangao@bol.com".to_string(),
                passw: "1234".to_string(),
                token: None
        };

        assert_eq!(validate_hash(user_data_correct), true);
        assert_ne!(validate_hash(user_data_incorrect), true);
    }
    
    #[test]
    fn deleting_test_user() {
        let mut conn: PgConnection = start_connection();
        
        let _ = diesel::delete(schema::users::dsl::users
        .filter(schema::users::dsl::user_name.eq("jango")))
        .execute(& mut conn);
    }

}
