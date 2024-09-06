use chrono::{NaiveDate, Duration, Utc, Datelike};
use diesel::{prelude::*, Insertable, PgConnection, Queryable};
use crate::schema;
use crate::database::conec::start_connection;
use rocket::serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Insertable, Deserialize, Serialize, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::todos)]
pub struct TodoItem {
    pub user_id: i32,
    pub todo_id: i32,
    pub creation_dt: NaiveDate,
    pub active_status: bool,
    pub description: String,
    pub set_dt: NaiveDate,
    pub color: String
}

impl TodoItem {
    pub fn new(usr_id: i32, descrip: String, dt: NaiveDate, set_color: String) -> bool {
        let mut conec: PgConnection = start_connection();

        let new_todo: TodoItem = 
            TodoItem{ 
                user_id: usr_id,
                todo_id: Self::gen_todo_id(),
                creation_dt: Utc::now().date_naive(),
                active_status: true,
                description: descrip,
                set_dt: dt,
                color: set_color
            };
        
        match diesel::dsl::insert_into(schema::todos::dsl::todos)
            .values(&new_todo)
            .execute(&mut conec) {
                Ok(_) => {
                    return true;
                },

                Err(_) => {
                    return false
                }
            }
    }

    pub fn gen_todo_id() -> i32 {
        let mut conec: PgConnection = start_connection();

        match schema::todos::dsl::todos
            .select(diesel::dsl::max(schema::todos::dsl::todo_id))
            .first::<Option<i32>>(&mut conec) {
                Ok(Some(last_id)) => {
                    return last_id + 1;
                },
                Ok(None) => {
                    return 1;
                },
                Err(_) => {
                    return 1;
                }
            }
    }

    pub fn get_month_todos(dt: NaiveDate, usr_id: i32, still_active: bool) -> Vec<Self> {
        let mut conec: PgConnection = start_connection();
        let first_month_day: NaiveDate = NaiveDate::from_ymd_opt(dt.year(), dt.month(), 1).unwrap();
        let last_month_day: NaiveDate =  NaiveDate::from_ymd_opt(dt.year(), dt.month() + 1, 1).unwrap() - Duration::days(1);

        return schema::todos::dsl::todos 
            .filter(schema::todos::dsl::user_id.eq(usr_id))
            .filter(schema::todos::dsl::set_dt.ge(first_month_day)
                .and(schema::todos::dsl::set_dt.le(last_month_day)))
            .filter(schema::todos::dsl::active_status.eq(still_active))
            .load::<TodoItem>(&mut conec).unwrap();
    }
} 



pub fn is_todo_owner(usr_id: i32, id_todo: i32) -> bool {
    let mut conec: PgConnection = start_connection();

    let owner = schema::todos::dsl::todos
        .select(schema::todos::dsl::user_id)
        .filter(schema::todos::dsl::todo_id.eq(id_todo))
        .first::<i32>(&mut conec).unwrap();

    if usr_id == owner {
        return true;
    } else {
        return false;
    }
}

pub fn delete_todo(id_todo: i32) -> bool {
    let mut conec: PgConnection = start_connection();

    match diesel::update(schema::todos::dsl::todos
        .filter(schema::todos::dsl::todo_id.eq(id_todo)))
        .set(schema::todos::dsl::active_status.eq(false))
        .execute(&mut conec).as_mut() {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        }
}

pub fn modify_todo(id_todo: i32, new_describe: String, new_dt: NaiveDate, new_color: String) -> bool {
    let mut conec: PgConnection = start_connection();

    match diesel::update(schema::todos::dsl::todos
        .filter(schema::todos::dsl::todo_id.eq(id_todo)))
        .set((
            schema::todos::dsl::description.eq(new_describe),
            schema::todos::dsl::set_dt.eq(new_dt),
            schema::todos::dsl::color.eq(new_color)
        ))
        .execute(&mut conec) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
}
