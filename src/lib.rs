pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use self::models::ErrorMatch;
pub fn get_errors() -> Vec<ErrorMatch> {
    use schema::errors::dsl::errors;
    let connection = establish_connection();
    let results = errors
        .load::<ErrorMatch>(&connection)
        .expect("Error loading matches");
    results
}

pub fn new_error(conn: &MysqlConnection, name: &str, identifier: &str) {
    use schema::errtype;
    let new_err = models::NewErrorType { name, identifier };
    let x = diesel::insert_into(errtype::table)
        .values(&new_err)
        .execute(conn);
    println!("New error result:::{:?}", x);
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
