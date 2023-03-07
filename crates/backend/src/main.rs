#[macro_use]
extern crate rocket;

use crate::database::Db;
use sea_orm_rocket::Database;

mod database;
mod routes;

/*
use bcrypt::DEFAULT_COST;
#[get("/encrypt?<password>")]
async fn encrypt(password: String) -> String {
    match bcrypt::hash(password, DEFAULT_COST) {
        Ok(str) => str,
        Err(_) => "Could not hash password".into(),
    }
}*/

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::auth::login])
        .attach(Db::init())
}
