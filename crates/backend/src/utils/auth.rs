use ::entity::token::{self as Token};
use base64::Engine;
use rand::rngs::OsRng;
use rand::RngCore;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use time::OffsetDateTime;

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

pub fn generate_token() -> String {
    let mut rng = OsRng::default();
    let mut result = [0; 256];
    rng.fill_bytes(&mut result);
    base64::engine::general_purpose::STANDARD.encode(result)
}
