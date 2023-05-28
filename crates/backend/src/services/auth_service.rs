use std::ops::Add;

use ::entity::token::{self as Token};
use base64::Engine;
use rand::{rngs::OsRng, RngCore};
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{prelude::*, ActiveValue};
use time::{Duration, OffsetDateTime};

use crate::error::CustomError;

pub async fn create_oauth_token(
    access_token: String,
    refresh_token: Option<String>,
    owner: i64,
    application_id: i64,
    scope: String,
    db: &DatabaseConnection,
) -> Result<OauthTokenResult, CustomError> {
    Token::Entity::delete_many()
        .filter(Token::Column::Owner.eq(owner))
        .filter(Token::Column::ApplicationId.eq(application_id))
        .exec(db)
        .await?;

    Token::ActiveModel {
        token: ActiveValue::Set(access_token.clone()),
        refresh: ActiveValue::Set(refresh_token.clone()),
        // This token is not meant to be used in the Authorization header, but in
        token_type: ActiveValue::Set("Bearer".into()),
        owner: ActiveValue::Set(owner),
        expire: ActiveValue::Set(OffsetDateTime::now_utc().add(Duration::days(7))),
        application_id: ActiveValue::Set(Some(application_id)),
        scope: ActiveValue::Set(scope.clone()),
    }
    .insert(db)
    .await?;

    Ok(OauthTokenResult {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: Duration::days(7).whole_seconds(),
        scope: scope,
    })
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OauthTokenResult {
    access_token: String,
    refresh_token: Option<String>,
    token_type: String,
    expires_in: i64,
    scope: String,
}

pub async fn is_valid_token(
    token: String,
    scopes: &[String],
    db: &DatabaseConnection,
) -> Result<bool, DbErr> {
    let token = Token::Entity::find_by_id(token).one(db).await?;

    if token.is_none() {
        return Ok(false);
    }
    let token = token.unwrap();

    if token.expire <= OffsetDateTime::now_utc() {
        return Ok(false);
    }

    let token_scopes = token.scope.split_ascii_whitespace();

    Ok(scopes
        .iter()
        .all(|scope| token_scopes.clone().into_iter().any(|s| s == scope)))
}

pub async fn get_token_user_id(token: String, db: &DatabaseConnection) -> Result<i64, DbErr> {
    let token = Token::Entity::find_by_id(token).one(db).await?;

    Ok(token.unwrap().owner)
}

pub fn generate_token(bits: usize) -> String {
    let mut rng = OsRng::default();
    let mut result = vec![0; bits];
    rng.fill_bytes(&mut result);
    base64::engine::general_purpose::STANDARD.encode(result)
}
