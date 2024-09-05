use chrono::{NaiveDate, Duration, Utc};
use crate::schema;
use crate::database::conec::start_conection;


pub struct TodoItem {
    user_id: i32,
    todo_id: i32,
    creation_dt: NaiveDate,
    active_status: bool,
    description: String,
    set_dt: NaiveDate,
    color: String
}

impl TodoItem {
    pub fn new(usr_id: i32, descrip: String, dt: NaiveDate, set_color: String) -> bool {
        let new_todo: TodoItem { 
                user_id: usr_id,
                todo_id: gen_todo_id(),
                creation_dt: Utc::now().data_naive(),
                active_status: true,
                description: descrip,
                set_dt: dt,
                color: set_color
            }
        
        match diesel::insert(schema::todos::dsl::todos)
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
        let last_id = schema::todos::dls::todos
            .select(diesel::max(schema::todos::dsl::todo_id))
            .first::<i32>(&mut conec);

        return last_id + 1;
    }

    pub fn get_month_todos(dt: NaiveDate, usr_id: i32, still_active: bool) -> Vec<Self> {
        let conec: PgConnection = start_conection();

        let first_month_day = NaiveDate::from_ymd(dt.year(), dt.month(), 1);
        let last_month_day =  NaiveDate::from_ymd(dt.year(), dt.month() + 1, 1) - Duration::days(1);

        return schema::todos::dsl::todos 
            .filter(schema::todos::dsl::user_id.eq(usr_id))
            .filter(schema::todos::dsl::set_dt.ge(first_month_day)
                .and(schema::todos::dsl::set_dt.le(last_month_day)))
            .filter(schema::todos::dsl::active_status.eq(still_active))
            .load::<Vec<TodoItem>>(&mut conec);
    }
} 

pub fn is_todo_owner(usr_id: i32, id_todo: i32) -> bool {
    let conec: PgConnection = start_conection();

    let owner = schema::todos::dsl::todos
        .select(schema::todos::dsl::user_id)
        .filter(schema::todos::dsl::todo_id.eq(id_todo))
        .first::<i32>(&mut conec);

    if user_id == owner {
        return true;
    } else {
        return false;
    }
}

pub fn delete_todo(id_todo: i32) -> bool {
    let conec: PgConnection = start_conection();

    match diesel::update(
        schema::todos::dsl::todos
            .filter(schema::todos::dsl::todo_id.eq(id_todo))
            .set(schema::todos::dsl::active_status.eq(false))
        )
        .execute(&mut conec) {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                return false;
            }
        }
}

pub fn modify_todo(id_todo: i32, new_describe: String, new_dt: NaiveDate, new_color: String) -> bool {
    let conec: PgConnection = start_conection();

    match diesel::update(
        schema::todos::dsl::todos
            .filter(schema::todos::dsl::todo_id.eq(id_todo))
            .set((
                schema::todos::dsl::description.eq(new_describe),
                schema::todos::dsl::set_dt.eq(new_dt),
                schema::todos::dsl::color.eq(new_color)
            ))
        )
        .execute(&mut conec) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
}
