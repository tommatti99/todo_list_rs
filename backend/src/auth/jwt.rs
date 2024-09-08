use chrono::{DateTime, Duration, Utc};
use diesel::PgConnection;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use std::env;
use rocket::serde::{Serialize, Deserialize};
use crate::database::conec::start_connection;
use crate::schema;
use diesel::prelude::*;


#[derive(Debug,Serialize, Deserialize)]
pub struct Claims {
    sub: String, //(subject): "user123" - Identifica o usuário ao qual o token pertence.
    exp: String, //(expiration time): 1716239022 - Define a data e hora de expiração do token (em segundos desde a época Unix).
    iat: String  //(issued at): 1716235422 - Define a data e hora em que o token foi emitido (em segundos desde a época Unix).
}

impl Claims {
    pub fn default(user_id: String) -> Self {
        return Claims {
            sub: user_id,
            exp: (Utc::now() + Duration::minutes(20)).to_string(),
            iat: Utc::now().to_string()
        };
    }
}

fn gen_encoding_jwt_key() -> EncodingKey {
    let secret = env::var("JWTSECRET").expect("Error on jwt");

    return EncodingKey::from_secret(secret.clone().as_bytes());
}

pub fn get_jwt(user_id: i32) -> String {
     match encode(&Header::new(Algorithm::RS256), &Claims::default(user_id.to_string()), &gen_encoding_jwt_key()) {
        Ok(token) => {
            return token;
        },
        Err(_) => {
            return "authentication error".to_string();
        }
    }
}

fn jwt_verify_its_on_database(usr_id: i32, token: &str) -> bool {
    let mut conec: PgConnection = start_connection();

    let jwt_code_db = schema::users::dsl::users
            .select(schema::users::dsl::jwt_code)
            .filter(schema::users::dsl::user_id.eq(usr_id))
            .first::<String>(&mut conec).unwrap();

    if jwt_code_db == token {
        return true;
    } else {
        return false;
    }
}

fn jwt_verify_expiration(decoded_token: TokenData<Claims>) -> bool {
    let expiration = DateTime::parse_from_str(&decoded_token.claims.exp.clone(), "%Y-%m-%d %H:%M:%S").expect("Parsing Error");

    if Utc::now() < expiration {
        return true;
    } else {
        return false;
    }
}

pub fn verify_jwt_token(usr_id: i32, token: &str) -> bool {
    let decoding_key: &DecodingKey = &DecodingKey::from_secret("secret".as_ref());

    if jwt_verify_its_on_database(usr_id, token) {
        match decode::<Claims>(&token, decoding_key, &Validation::default()) {
            Ok(decoded_token) => { 
                return jwt_verify_expiration(decoded_token);
            },
            Err(_) => {
                return false;
            }
        }
    } else {
        return false;
    }
}


