use crate::database::Db;
use ::entity::token;
use ::entity::user;
use base64::Engine;
use bcrypt::BcryptError;
use rand::rngs::OsRng;
use rand::Error as RandError;
use rand::RngCore;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::serde_json;
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginSuccess {
    token: String,
    token_type: String,
    expires_in: i64,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for LoginSuccess {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let message = serde_json::to_string(&self)
            .unwrap_or("{ \"error\": \"Internal server error\" }".to_string());

        Response::build()
            .header(ContentType::JSON)
            .status(Status::Ok)
            .sized_body(message.len(), Cursor::new(message))
            .ok()
    }
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Invalid Credentials")]
    InvalidCredentials,
    #[error("Db error: {0}")]
    DbError(#[from] DbErr),
    #[error("bCrypt error: {0}")]
    BCryptError(#[from] BcryptError),
    #[error("Rand error: {0}")]
    RandError(#[from] RandError),
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

#[post("/auth/login", data = "<input>")]
pub async fn login(
    db: Connection<'_, Db>,
    input: Json<LoginData>,
) -> Result<LoginSuccess, LoginError> {
    let db = db.into_inner();

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(input.email.clone()))
        .one(db)
        .await?;

    if user.is_none() {
        return Err(LoginError::InvalidCredentials);
    }
    let user = user.unwrap();
    let correct_password = bcrypt::verify(&input.password, &user.password)?;

    if !correct_password {
        return Err(LoginError::InvalidCredentials);
    }

    let mut rng = OsRng::default();
    let mut result = [0; 256];
    rng.try_fill_bytes(&mut result)?;
    let access_token = base64::engine::general_purpose::STANDARD.encode(result);

    let token = token::ActiveModel {
        token: ActiveValue::Set(access_token.clone()),
        refresh: ActiveValue::Set(None),
        // This token is not meant to be used in the Authorization header, but in
        token_type: ActiveValue::Set("app_token".into()),
        owner: ActiveValue::Set(user.id),
        expire: ActiveValue::Set(OffsetDateTime::now_utc().add(Duration::days(7))),
        client_id: ActiveValue::Set(None),
        scope: ActiveValue::Set("".into()),
    };
    token.insert(db).await?;

    Ok(LoginSuccess {
        token: access_token,
        token_type: "app_token".into(),
        expires_in: Duration::days(7).whole_seconds(),
    })
}