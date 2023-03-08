#[macro_use]
extern crate rocket;

use crate::database::Db;
use sea_orm_rocket::Database;

mod database;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::auth::login::login])
        .attach(Db::init())
}
