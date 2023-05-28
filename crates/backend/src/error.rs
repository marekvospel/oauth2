use std::io::Cursor;

use redis::RedisError;
use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Request, Response,
};
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Internal server error")]
    Simple,

    #[error("{1}")]
    Custom(Status, String),

    #[error("Db error: {0}")]
    DbError(#[from] DbErr),

    #[error("Redis error: {0}")]
    RedisError(#[from] RedisError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (status, message) = match self {
            CustomError::Custom(status, message) => (status, message),
            _ => {
                error!("{self}");
                (Status::InternalServerError, "Internal server error".into())
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
