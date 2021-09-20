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

use schema::{errors, whitelist};

pub enum MatchType {
    TypeError,
    TypeWhitelist,
}

pub fn get_error_by_id(conn: &MysqlConnection, error_id: i32) -> Option<ErrorMatch> {
    let x = errors::table.find(error_id).load::<ErrorMatch>(conn);
    match x {
        Err(_) => return None,
        Ok(v) => {
            if v.len() == 0 {
                return None
            }
            return Some(v[0].clone());
        }
    }
}


pub fn insert(conn: &MysqlConnection, matching_string: String, reference_url: String, reference_case: String) {
    let new_err = models::NewErrorMatch { matching_string, reference_case, reference_url };
    let x = diesel::insert_into(errors::table)
        .values(&new_err)
        .execute(conn);
    println!("New INSERTION result:::{:?}", x);
}

pub fn insert_into_whitelist(conn: &MysqlConnection, matching_string: String, reference_url: String, reference_case: String) {
    let new_err = models::NewWhitelistMatch { matching_string, reference_case, reference_url };
    let x = diesel::insert_into(whitelist::table)
        .values(&new_err)
        .execute(conn);
    println!("New INSERTION result:::{:?}", x);
}

pub fn get_errors() -> Vec<ErrorMatch> {
    use schema::errors::dsl::errors;
    let connection = establish_connection();
    let results = errors
        .load::<ErrorMatch>(&connection)
        .expect("Error loading matches");
    results
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
