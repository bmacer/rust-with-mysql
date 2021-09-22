use diesel::mysql::MysqlConnection;
use super::schema::diagnostic_files;
use super::models::{DiagnosticFile, NewDiagnosticFile};
use crate::diesel::{QueryDsl, RunQueryDsl,ExpressionMethods};
use super::schema::diagnostic_files::dsl::{
    diagnostic_files as df,
    diag_url,
};

/// Retrieve diagnostic file from URL
/// If none, insert and try again (which should work)
pub fn get_or_insert_diagnostic_file_by_url(conn: &MysqlConnection, durl: String) 
    -> Option<DiagnosticFile> 
    {
        // Check if it exists, if so return it
        let r = df.filter(
            diag_url.eq(&durl))
            .load::<DiagnosticFile>(conn)
            .expect("error loading diagnostic file (1)");
        if r.len() > 0 {
            return Some(r[0].clone());
        }

        // If not, insert it
        insert_new_diagnostic_file(conn, durl.clone(), None);

        // Then check again, should be good now
        let r = df.filter(
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
    url: String, 
    sr: Option<String>)
    {
        let new_diag = NewDiagnosticFile { diag_url: url, sr };
        let x = diesel::insert_into(diagnostic_files::table)
            .values(&new_diag)
            .execute(conn);
        println!("Invidual result insert:::{:?}", x);
    }
