pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Post, NewPost};

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    let x = diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn);

    println!("x: {:?}", x);
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}