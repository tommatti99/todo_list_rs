use chrono::{NaiveDate,Utc};
use diesel::PgConnection;
use diesel::prelude::*;
use crate::database::conec::start_connection;
use rand::Rng;
use crate::schema;

#[allow(dead_code)]
pub fn verify_recover_code_is_valid(usr_id: i32, recover_code: i32) -> bool {
    let mut conec: PgConnection = start_connection();

    match schema::users::dsl::users
        .select((
            schema::users::dsl::recover_code,
            schema::users::dsl::recover_code_exp))
        .filter(schema::users::dsl::user_id.eq(usr_id))
        .first::<(i32, NaiveDate)>(&mut conec) {
            Ok((rec_code, rec_code_exp)) => {
                let now = Utc::now().naive_utc();

                if rec_code == recover_code && now < rec_code_exp.into() {
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => false,
        }
}

pub fn gen_recover_code(usr_id: i32) -> i32 {
    let mut conec: PgConnection = start_connection();
    let mut range: rand::prelude::ThreadRng = rand::thread_rng();
    let recover_code: i32 = range.gen_range(100000..1000000);
    let expiration = (Utc::now() + chrono::Duration::minutes(20)).naive_utc().date(); // Define a data de expiração

    let _ = diesel::update(schema::users::dsl::users)
        .filter(schema::users::dsl::user_id.eq(usr_id))
        .set((
            schema::users::dsl::recover_code.eq(recover_code),
            schema::users::dsl::recover_code_exp.eq(expiration),
        ))
        .execute(&mut conec);


    return recover_code;
}