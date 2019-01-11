#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate r2d2_mysql;
extern crate serde;

//mod expanded;
mod routes;
mod schema;
mod tables;

use crate::routes::{get, post};
use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};

pub const HOST: &str = "localhost:7999";
#[database("mysql")]
pub struct DBConn(MysqlConnection);

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                get::homepage,
                get::forumpage,
                get::threadpage,
                post::comment,
                post::thread
            ],
        ).attach(DBConn::fairing())
        .attach(Template::fairing())
        .launch();
}
