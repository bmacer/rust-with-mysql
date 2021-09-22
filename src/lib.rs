pub mod schema;
pub mod models;
pub mod tests;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

// pub enum MatchType {
//     TypeError,
//     TypeWhitelist,
// }

// pub use diesel::expression::dsl::sql;
// pub use diesel::sql_query;

pub mod error_results;
pub mod error_matches;
pub mod diagnostic_file;

/// Get Mysql connection
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}