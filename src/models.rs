// use super::schema::posts;
// use super::schema::errtype;
use super::schema::errors;
use super::schema::whitelist;

#[derive(Queryable)]
pub struct ErrorMatch {
    pub id: i32,
    pub matching_string: String,
    pub reference_url: String,
    pub reference_case: String,
}

#[derive(Insertable)]
#[table_name="errors"]
pub struct NewErrorMatch {
    pub matching_string: String,
    pub reference_url: String,
    pub reference_case: String,
}

#[derive(Insertable)]
#[table_name="whitelist"]
pub struct NewWhitelistMatch {
    pub matching_string: String,
    pub reference_url: String,
    pub reference_case: String,
}

// #[derive(Queryable)]
// pub struct ErrorType {
//     pub id: i32,
//     pub name: String,
//     pub identifier: String,
// }

// #[derive(Insertable)]
// #[table_name="errtype"]
// pub struct NewErrorType<'a> {
//     // pub id: i32,
//     pub name: &'a str,
//     pub identifier: &'a str,
// }


// #[derive(Insertable)]
// #[table_name="posts"]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }

// #[derive(Queryable)]
// pub struct Post {
//     pub id: i32,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }