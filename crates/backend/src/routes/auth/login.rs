use crate::database::Db;
use crate::utils::auth::generate_token;
use ::entity::token;
use ::entity::user;
use argon2::password_hash::Error as ArgonError;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::SameSite;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Request, Response};
use sea_orm::*;
use sea_orm_rocket::Connection;
use std::io::Cursor;
use std::ops::Add;
use thiserror::Error;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginData {
    email: String,
    password: String,
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Invalid Credentials")]
    InvalidCredentials,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Db error: {0}")]
    DbError(#[from] DbErr),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for LoginError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (status, message) = match self {
            LoginError::InvalidCredentials => (Status::Unauthorized, "Invalid credentials"),
            _ => {
                error!("{self}");
                (Status::InternalServerError, "Internal server error")
            }
        };
        let message = format!("{{ \"error\": \"{message}\" }}");

        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(message.len(), Cursor::new(message))
            .ok()
    }
}

#[post("/api/auth/login", data = "<input>")]
pub async fn login(
    db: Connection<'_, Db>,
    input: Json<LoginData>,
    cookies: &CookieJar<'_>,
) -> Result<(), LoginError> {
    let db = db.into_inner();

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(input.email.clone()))
        .one(db)
        .await?;

    if user.is_none() {
        return Err(LoginError::InvalidCredentials);
    }
    let user = user.unwrap();
    let hash = match PasswordHash::new(&user.password) {
        Ok(h) => h,
        Err(_) => return Err(LoginError::InternalServerError),
    };
    let correct_password = Argon2::default().verify_password(input.password.as_bytes(), &hash);

    if let Err(e) = correct_password {
        if let ArgonError::Password = e {
            return Err(LoginError::InvalidCredentials);
        } else {
            return Err(LoginError::InternalServerError);
        }
    }

    let access_token = generate_token();

    let token = token::ActiveModel {
        token: ActiveValue::Set(access_token.clone()),
        refresh: ActiveValue::Set(None),
        // This token is not meant to be used in the Authorization header, but in
        token_type: ActiveValue::Set("Bearer".into()),
        owner: ActiveValue::Set(user.id),
        expire: ActiveValue::Set(OffsetDateTime::now_utc().add(Duration::days(7))),
        application_id: ActiveValue::Set(None),
        scope: ActiveValue::Set("me".into()),
    };
    token.insert(db).await?;

    let mut token_cookie = Cookie::new("token", access_token);
    token_cookie.set_http_only(true);
    token_cookie.set_same_site(SameSite::Strict);
    token_cookie.set_expires(OffsetDateTime::now_utc().add(Duration::days(7)));

    cookies.add(token_cookie);

    Ok(())
}
