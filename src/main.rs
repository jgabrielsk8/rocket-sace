#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod config;
mod schema;
mod majad;

use rocket_contrib::json::JsonValue;
use rocket_contrib::databases::{database, diesel::PgConnection};

use majad::routes;


#[database("sace_rocket")]
pub struct DbConn(PgConnection);


#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    rocket::custom(config::from_env())
        .attach(DbConn::fairing())
        .mount("/majad", routes![routes::create_clase, routes::get_clases])
        .register(catchers![not_found])
        .launch();
}
