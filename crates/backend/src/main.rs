#![allow(clippy::let_unit_value)]

#[macro_use]
extern crate rocket;

use database::RedisFairing;
use rocket::fairing::AdHoc;
use rocket::{fairing, Build, Rocket};
use sea_orm_rocket::Database;

use crate::database::Db;
use migration::MigratorTrait;

mod database;
mod routes;
mod utils;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    redis::Client::open("redis://127.0.0.1/").unwrap();

    rocket::build()
        .attach(Db::init())
        .attach(RedisFairing)
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount(
            "/",
            routes![
                routes::auth::login::login,
                routes::auth::authorize::authorize,
                routes::auth::authorize::token,
            ],
        )
}
