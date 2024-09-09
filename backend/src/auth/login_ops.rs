use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},Argon2, PasswordHash};
use diesel::PgConnection;
use diesel::prelude::*;
use crate::schema::users;
use crate::database::conec::start_connection;
use crate::auth::login_models::LoginUserRequest;
use crate::schema;
use crate::schema::users::salt_hash_bytes;
use crate::auth::jwt;



fn new_salt() -> SaltString {
    return SaltString::generate(&mut OsRng);
}

pub fn new_hash(passw: &String) -> Result<String, String> {
    let salt: SaltString = new_salt();
    match Argon2::default().hash_password(passw.as_bytes(), &salt) {
        Ok(hash) => {
            return Ok(hash.to_string())
        },
        Err(_) => {
            return Err("ERROR".to_string());
        }
    }
}

pub fn validate_hash(login_data: LoginUserRequest) -> bool {
    let argon2id = Argon2::default();
    let user_hash_on_db = get_salt_hash_on_db(login_data.usr_id).unwrap();
    
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

pub fn get_salt_hash_on_db(usr_id: i32) -> Result<String, String> {
    let mut conec: PgConnection = start_connection();
    
    match schema::users::dsl::users
            .filter(schema::users::dsl::user_id.eq(usr_id))
            .select(salt_hash_bytes)
            .first::<Vec<u8>>(&mut conec) {
                Ok(hash_vec) => {
                    match std::str::from_utf8(&hash_vec.as_slice()) {
                        Ok(hash_str) => {
                            return Ok(hash_str.to_string());
                        },
                        Err(_) => {
                            return Err("ERROR".to_string());
                        }
                    }
                },
                Err(_) => {
                    return Err("ERROR".to_string());
                }
    };
}

pub fn verify_user_exists(usr_email: String) -> Result<bool, String> {
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
            return Err("ERROR".to_string());
        }
    }
}


pub fn change_passw_by_token(usr_id: i32, usr_email: String, token: String, new_passw: String) -> bool {
    if verify_user_exists(usr_email.clone()).unwrap() {
        if jwt::verify_jwt_token(usr_id, &token) {
            let mut conec: PgConnection = start_connection();

            let new_passw_hash = new_hash(&new_passw).unwrap().to_string().as_bytes().to_vec();
            
            match diesel::update(schema::users::dsl::users)
                    .filter(users::dsl::email.eq(usr_email))
                    .set(users::salt_hash_bytes.eq(new_passw_hash))
                    .execute(&mut conec) {
                Ok(_) => {
                    return true;
                },
                Err(_) => {
                    return false;
                }
            }
        } else {
            return false; 
        }
    } else {
        return false;
    }
}



pub fn create_new_user(user_name_new_user: String, email_new_user: String, passw_new_user: &String) -> bool {
    let mut conec: PgConnection = start_connection();
    let mut new_user_id: i32;

    match schema::users::dsl::users
    .select(diesel::dsl::max(schema::users::dsl::user_id))
    .first::<Option<i32>>(&mut conec)
        .expect("Error Creating a new user") {
            Some(max_user_id) => {
                new_user_id = max_user_id;
            },
            None => {
                new_user_id = 0;
            }
        }
    new_user_id = new_user_id + 1;


    if !verify_user_exists(email_new_user.clone()).unwrap() {
        match diesel::insert_into(users::table)
            .values((
                schema::users::dsl::email.eq(email_new_user),
                schema::users::dsl::user_id.eq(new_user_id),
                schema::users::dsl::user_name.eq(user_name_new_user),
                schema::users::dsl::salt_hash_bytes.eq(new_hash(passw_new_user).unwrap().to_string().as_bytes().to_vec()),
                ))
            .execute(&mut conec) {
                Ok(_) => {
                    return true;
                },
                Err(_) => {
                    return false;
                }
            }
    } else {
        return false;
    }
}