extern crate dd;
extern crate diesel;

use std::io::stdin;
use std::env::args;
use self::dd::*;

fn main() {
    let connection = establish_connection();
    let e = get_error_by_id(&connection, 2);
}
