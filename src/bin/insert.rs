extern crate dd;
extern crate diesel;

use std::io::stdin;
use std::env::args;
use self::dd::*;

fn main() {
    let connection = establish_connection();
    let mut written_count = 0;

    let target = args().nth(1).expect("Expected error or whitelist");
    println!("{}", target);
    if target != "error" && target != "whitelist" {
        panic!("Expected arg: error or whitelist");
    }
    println!("Diagnostic URL:");
    let mut reference_url = String::new();
    stdin().read_line(&mut reference_url).unwrap();
    let reference_url = String::from(&reference_url[..(reference_url.len() - 1)]); // Drop the newline character
    
    println!("Reference Case:");
    let mut reference_case = String::new();
    stdin().read_line(&mut reference_case).unwrap();
    let reference_case = String::from(&reference_case[..(reference_case.len() - 1)]); // Drop the newline character

    loop {
        let refurl = reference_url.clone();
        let refcase = reference_case.clone();
        println!("Matching String (enter if done):");
        let mut matching_string = String::new();
        stdin().read_line(&mut matching_string).unwrap();
        if matching_string == "" || matching_string == "\n" {
            break;
        }
        let matching_string = String::from(&matching_string[..(matching_string.len() - 1)]); // Drop the newline character

        written_count += 1;
        if target == "error" {
            insert(&connection, matching_string, refurl, refcase);
            println!("inserting into error");
        } else if target == "whitelist" {
            insert_into_whitelist(&connection, matching_string, refurl, refcase);
            println!("inserting into whitelist");
        } else {
            panic!("this shouldn't be able to happen, i checked for this already");
        }
    }
    println!("Wrote {} errors", written_count);
}
