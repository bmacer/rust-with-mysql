extern crate dd;
extern crate diesel;

use std::io::stdin;
use self::dd::*;

fn main() {
    let connection = establish_connection();
    println!("Error Name:");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nMatching String:");
    let mut matching_string = String::new();
    stdin().read_line(&mut matching_string).unwrap();
    new_error(&connection, name, &matching_string);
    println!("Saved draft");
}