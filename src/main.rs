#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate r2d2_mysql;
extern crate serde;
extern crate toml;

mod routes;
mod schema;
mod settings;
mod tables;

use crate::{
    routes::{get, post},
    settings::Settings,
};
use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};

pub const HOST: &str = "localhost:7999";
#[database("mysql")]
pub struct DBConn(MysqlConnection);

fn main() -> Result<(), Box<::std::error::Error>> {
    let settings = Settings::load()?;
    dbg!(&settings);
    rocket::ignite()
        .manage(settings)
        .mount(
            "/",
            routes![
                get::homepage,
                get::forumpage,
                get::threadpage,
                get::categorypage,
                post::comment,
                post::thread
            ],
        ).attach(DBConn::fairing())
        .attach(Template::fairing())
        .launch();
    Ok(())
}
