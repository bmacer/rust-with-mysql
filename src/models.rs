// use super::schema::posts;
// use super::schema::errtype;
use super::schema::errors;
use super::schema::whitelist;
use super::schema::error_results;
use super::schema::diagnostic_files;

/// GET for errors
#[derive(Queryable, Clone, Debug)]
pub struct ErrorMatch {
    pub id: i32,
    pub matching_string: String,
    pub reference_url: String,
    pub reference_case: String,
}
/// INSERT for errors
#[derive(Insertable)]
#[table_name="errors"]
pub struct NewErrorMatch {
    pub matching_string: String,
    pub reference_url: String,
    pub reference_case: String,
}

/// GET for error_results
#[derive(Queryable, Clone, Debug)]
pub struct ErrorResult {
    pub id: i32,
    pub matching_string_id: i32,
    pub diagnostic_file_id: i32,
    pub count: i32,
}
/// INSERT for error_results
#[derive(Insertable)]
#[table_name="error_results"]
pub struct NewErrorResult {
    pub matching_string_id: i32,
    pub diagnostic_file_id: i32,
    pub count: i32,
}

/// GET for diagnostic_files
#[derive(Queryable, Clone, Debug)]
pub struct DiagnosticFile {
    pub id: i32,
    pub diag_url: String,
    pub sr: Option<String>,
}
/// INSERT for diagnostic_files
#[derive(Insertable)]
#[table_name="diagnostic_files"]
pub struct NewDiagnosticFile {
    pub diag_url: String,
    pub sr: Option<String>,
}

/// INSERT for whitelist
#[derive(Insertable)]
#[table_name="whitelist"]
pub struct NewWhitelistMatch {
    pub matching_string: String,
    pub reference_url: String,
    pub reference_case: String,
}
