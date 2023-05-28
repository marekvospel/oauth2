#![allow(clippy::let_unit_value)]

#[macro_use]
extern crate rocket;

use base64::Engine;
use database::RedisFairing;
use rocket::fairing::AdHoc;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::{fairing, Build, Request, Rocket};
use sea_orm_rocket::Database;

use crate::database::Db;
use migration::MigratorTrait;

mod database;
mod error;
mod routes;
mod services;

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

pub enum BasicAuth {
    NotPresent,

    Present(BasicAuthCredentials),
}

pub struct BasicAuthCredentials {
    pub username: String,

    pub password: String,
}

impl BasicAuth {
    pub fn get_username(&self) -> Option<String> {
        match self {
            BasicAuth::NotPresent => None,
            BasicAuth::Present(auth) => Some(auth.username.clone()),
        }
    }

    pub fn get_password(&self) -> Option<String> {
        match self {
            BasicAuth::NotPresent => None,
            BasicAuth::Present(auth) => Some(auth.password.clone()),
        }
    }
}

impl From<String> for BasicAuth {
    fn from(value: String) -> Self {
        if value.len() < 7 || &value[..6] != "Basic " {
            return BasicAuth::NotPresent;
        }

        let decoded = String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(&value[6..])
                .unwrap(),
        )
        .unwrap();

        match decoded.split_once(":") {
            Some((username, password)) => BasicAuth::Present(BasicAuthCredentials {
                username: username.to_string(),
                password: password.to_string(),
            }),
            None => BasicAuth::NotPresent,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let headers: Vec<_> = request.headers().get("Authorization").collect();

        Outcome::Success(match headers.len() {
            1 => BasicAuth::from(headers[0].to_string()),
            _ => BasicAuth::NotPresent,
        })
    }
}
