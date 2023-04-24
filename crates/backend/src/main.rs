#![allow(clippy::let_unit_value)]

#[macro_use]
extern crate rocket;

use rocket::async_trait;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::{fairing, Build, Request, Rocket};
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
        .mount("/", routes![routes::auth::login::login, routes::me::me,])
}

pub struct Token(String);

#[async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization").unwrap_or("");
        Outcome::Success(Token(token.to_string()))
    }
}
