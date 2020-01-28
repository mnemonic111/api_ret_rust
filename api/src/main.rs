#[macro_use]
extern create diesel;
#[macro_use]
extern create diesel_codegen;
extern create dotenv;

use dotenv::dotenv;
use std::env;

mod schema;
mod models;



fn main() {
    println!("Hello, world!");
}
