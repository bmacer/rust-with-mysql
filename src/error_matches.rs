use diesel::mysql::MysqlConnection;
use super::schema::errors;
use super::schema::whitelist;
use super::models::{ErrorMatch, NewErrorMatch, NewWhitelistMatch};
use crate::diesel::{
    QueryDsl, 
    RunQueryDsl,
    // ExpressionMethods
};
use super::schema::errors::dsl::{
    errors as e,
};

/// SELECT * FROM errors
pub fn get_errors(conn: &MysqlConnection) -> Vec<ErrorMatch> {
    let results = e
        .load::<ErrorMatch>(conn)
        .expect("Error loading matches");
    results
}

/// INSERT INTO whitelist
pub fn insert_into_whitelist(conn: &MysqlConnection, matching_string: String, reference_url: String, reference_case: String) {
    let new_err = NewWhitelistMatch { matching_string, reference_case, reference_url };
    let _x = diesel::insert_into(whitelist::table)
        .values(&new_err)
        .execute(conn);
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
pub fn insert_into_errors(
    conn: &MysqlConnection, 
    matching_string: String, 
    reference_url: String, 
    reference_case: String) {
        let new_err = NewErrorMatch { 
            matching_string, reference_case, reference_url };
        let _x = diesel::insert_into(errors::table)
            .values(&new_err)
            .execute(conn);
    }
