#![feature(decl_macro, proc_macro_hygiene)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use diesel_migrations::embed_migrations;
embed_migrations!("migrations/");

mod api;
mod schema;
mod connection;
mod cors;


fn main() {
    dotenv().ok();
    api::router::create_routes();
}
