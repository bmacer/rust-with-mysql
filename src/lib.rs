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

use self::models::ErrorMatch;
use self::models::DiagnosticFile;

use schema::{errors, whitelist, diagnostic_files};

pub enum MatchType {
    TypeError,
    TypeWhitelist,
}

pub use diesel::expression::dsl::sql;
pub use diesel::sql_query;

/// Retrieve diagnostic file from URL
/// If none, insert and try again (which should work)
pub fn get_or_insert_diagnostic_file_by_url(conn: &MysqlConnection, durl: String) 
    -> Option<DiagnosticFile> 
    {
        use schema::diagnostic_files::dsl::{
            diagnostic_files, diag_url
        };
        // Check if it exists, if so return it
        let r = diagnostic_files.filter(
            diag_url.eq(&durl))
            .load::<DiagnosticFile>(conn)
            .expect("error loading diagnostic file (1)");
        if r.len() > 0 {
            return Some(r[0].clone());
        }

        // If not, insert it
        insert_new_diagnostic_file(conn, durl.clone(), None);

        // Then check again, should be good now
        let r = diagnostic_files.filter(
            diag_url.eq(&durl))
            .load::<DiagnosticFile>(conn)
            .expect("error loading diagnostic file (2)");
        
        if r.len() > 0 {
            return Some(r[0].clone());
        }

        // This shouldn't be possible
        return None;
    }

/// INSERT INTO diagnostic_files...
pub fn insert_new_diagnostic_file(
    conn: &MysqlConnection, 
    diag_url: String, 
    sr: Option<String>)
    {
        let new_diag = models::NewDiagnosticFile { diag_url, sr };
        let x = diesel::insert_into(diagnostic_files::table)
            .values(&new_diag)
            .execute(conn);
        println!("Invidual result insert:::{:?}", x);
    }

/// INSERT INTO error_results
pub fn insert_result(
    conn: &MysqlConnection,
    matching_string_id: i32,
    diagnostic_file_id: i32,
    count: i32)
    {
        use schema::error_results;
        use models::NewErrorResult;

        let new_error_result = NewErrorResult { 
            matching_string_id, 
            diagnostic_file_id, 
            count 
        };
        let x = diesel::insert_into(error_results::table)
            .values(&new_error_result)
            .execute(conn);
        println!("New result insert:::{:?}", x);
}

/// SELECT * FROM errors WHERE id=error_id
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

/// INSERT INTO errors
pub fn insert(conn: &MysqlConnection, matching_string: String, reference_url: String, reference_case: String) {
    let new_err = models::NewErrorMatch { matching_string, reference_case, reference_url };
    let x = diesel::insert_into(errors::table)
        .values(&new_err)
        .execute(conn);
    println!("New INSERTION result:::{:?}", x);
}

/// INSERT INTO whitelist
pub fn insert_into_whitelist(conn: &MysqlConnection, matching_string: String, reference_url: String, reference_case: String) {
    let new_err = models::NewWhitelistMatch { matching_string, reference_case, reference_url };
    let x = diesel::insert_into(whitelist::table)
        .values(&new_err)
        .execute(conn);
    println!("New INSERTION result:::{:?}", x);
}

/// SELECT * FROM errors
pub fn get_errors() -> Vec<ErrorMatch> {
    use schema::errors::dsl::errors;
    let connection = establish_connection();
    let results = errors
        .load::<ErrorMatch>(&connection)
        .expect("Error loading matches");
    results
}

/// Get Mysql connection
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}




