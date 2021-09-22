#[test]
fn it_works() {
    use super::get_errors;
    let x = get_errors();
    for i in x {
        println!("ID {} NAME {}", i.id, i.matching_string);
    }
    let c = super::establish_connection();
    let x = super::insert_new_diagnostic_file(
        &c,
        String::from("def"),
        None);
    println!("x: {:?}", x);
    // assert!(false);
}

#[test]
fn get_diagnostic_file_by_url() {
    use super::{get_or_insert_diagnostic_file_by_url, establish_connection};

    let c = establish_connection();
    let d = get_or_insert_diagnostic_file_by_url(
        &c, String::from("abcdefg"));
    println!("d: {:?}", d);
    // assert!(false);
}

#[test]
fn test_insert_result() {
    use super::{
        insert_result,
        establish_connection};
    let c = establish_connection();
    let r = insert_result(&c, 101, 200, 300);
    let r = insert_result(&c, 101, 201, 3000);
    let r = insert_result(&c, 101, 201, 3001);
    assert!(false);
}
