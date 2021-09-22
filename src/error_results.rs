use diesel::mysql::MysqlConnection;
use super::schema::error_results;
use super::models::{ErrorResult, NewErrorResult};
use crate::diesel::{QueryDsl, RunQueryDsl,ExpressionMethods};
use super::schema::error_results::dsl::{
    error_results as er,
    matching_string_id,
    diagnostic_file_id,
    id,
};

/// INSERT INTO error_results
pub fn insert_into_error_results(
    conn: &MysqlConnection,
    match_id: i32,
    diag_id: i32,
    count: i32)
    {
    let new_error_result = NewErrorResult { 
        matching_string_id: match_id, 
        diagnostic_file_id: diag_id, 
        count 
        };
    let _x = diesel::insert_into(error_results::table)
        .values(&new_error_result)
        .execute(conn);
}

/// SELECT * FROM error_result WHERE match_id=x AND diag_id=y
pub fn get_error_result_by_match_id_and_diag_id(
    conn: &MysqlConnection,
    match_id: i32,
    diag_id: i32,
) -> Option<ErrorResult> {
    
    let r = er
        .filter(matching_string_id.eq(match_id))
        .filter(diagnostic_file_id.eq(diag_id))
        .load::<ErrorResult>(conn)
        .expect("Error loading ErrorResult");
    if r.len() > 0 {
        return Some(r[0].clone());
    }
    return None
}

// DELETE FROM error_results WHERE id=x;
pub fn delete_error_result(
    conn: &MysqlConnection,
    id_to_delete: i32,
) {
    let _x = diesel::delete(er.filter(id.eq(id_to_delete)))
        .execute(conn);
}

/// Check for Entry in error_results
/// Delete if found, then insert
pub fn insert_or_override_error_result_entry(
    conn: &MysqlConnection,
    match_id: i32,
    diag_id: i32,
    count: i32,
) {
    let initial_result = get_error_result_by_match_id_and_diag_id(
        conn, match_id, diag_id);
    if initial_result.is_some() {
        delete_error_result(conn, initial_result.unwrap().id);
    }
    insert_into_error_results(conn, match_id, diag_id, count);
}