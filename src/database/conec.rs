use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::CustomErrors;

pub fn start_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE MUST BE SET");

    return PgConnection::establish(&database_url)
        .expect(CustomErrors::UnavailableDataBase.describe());
}

#[cfg(test)]
mod test_conec {
    use super::*;
    use std::any::type_name_of_val;

    #[test]
    fn connected(){
        let connec = start_connection();
        assert_eq!(type_name_of_val(&connec), "diesel::pg::connection::PgConnection") 
    }
}
