#![allow(clippy::let_unit_value)]

#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::{fairing, Build, Rocket};
use sea_orm_rocket::Database;

use crate::database::Db;
use migration::MigratorTrait;

mod database;
mod routes;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount(
            "/",
            routes![
                routes::auth::login::login,
                routes::auth::oauth2::discord_redirect,
                routes::auth::oauth2::discord_authorize
            ],
        )
}
