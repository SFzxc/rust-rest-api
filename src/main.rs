#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

use dotenv::dotenv;

mod product;
mod schema;
mod connection;

fn main() {
    dotenv().ok();
    product::router::create_routes();
}
