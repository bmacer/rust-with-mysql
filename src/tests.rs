#[test]
fn it_works() {
    use super::error_matches::get_errors;
    use super::establish_connection;

    let c = establish_connection();
    let x = get_errors(&c);
    for i in x {
        println!("ID {} NAME {}", i.id, i.matching_string);
    }
    let c = super::establish_connection();
    let x = super::diagnostic_file::insert_new_diagnostic_file(
        &c,
        String::from("def"),
        None);
    println!("x: {:?}", x);
    // assert!(false);
}

#[test]
fn get_diagnostic_file_by_url() {
    use super::{diagnostic_file::get_or_insert_diagnostic_file_by_url, establish_connection};

    let c = establish_connection();
    let d = get_or_insert_diagnostic_file_by_url(
        &c, String::from("abcdefg"));
    println!("d: {:?}", d);
    // assert!(false);
}

#[test]
fn test_insert_result() {
    use super::{
        error_results::insert_into_error_results,
        establish_connection};
    let c = establish_connection();
    insert_into_error_results(&c, 101, 200, 300);
    insert_into_error_results(&c, 101, 201, 300);
    insert_into_error_results(&c, 101, 202, 300);
    // assert!(false);
}

#[test]
pub fn test_get_error_result_by_match_id_and_diag_id() {
    println!("testing test_get_error_result_by_match_id_and_diag_id");
    use super::{
        error_results::get_error_result_by_match_id_and_diag_id,
        establish_connection};
    let c = establish_connection();
    let r = get_error_result_by_match_id_and_diag_id(&c, 101, 200);
    println!("R: {:?}", r);
    // assert!(r.is_none());
    // assert!(false);
}

#[test]
pub fn test_delete_error_result() {
    use super::{
        error_results::delete_error_result,
        establish_connection};
    let c = establish_connection();
    delete_error_result(&c, 1);
    // assert!(false);
}

#[test]
pub fn test_insert_or_override_error_result_entry() {
    println!("testing insert_or_override_error_result_entry");
    use super::{
        error_results::insert_or_override_error_result_entry,
        establish_connection};

    let c = establish_connection();
    insert_or_override_error_result_entry(&c, 101, 201, 301);


    assert!(false);
}