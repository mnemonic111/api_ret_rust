use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub #[derive(Queryable)]
struct Book {
    pub: Id :i32, 
    pub title : String,
    pub author: String, 
    pub published: bool
}