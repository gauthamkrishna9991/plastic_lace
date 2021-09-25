// Webserver Library
#[macro_use]
extern crate rocket;

// All Database-specific libraries required
use rocket_sync_db_pools::{database, diesel};

// API Module
mod api;

// AUTH Module

#[database("my_database")]
struct MainDB(diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(api::stage())
        .mount("/", routes![index])
}
