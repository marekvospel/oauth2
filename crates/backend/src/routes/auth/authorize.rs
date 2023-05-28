use base64::Engine;
use entity::application;
use rand::{rngs::OsRng, RngCore};
use redis::AsyncCommands;
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::{
    http::Status,
    serde::{
        json::{serde_json, Json},
        Deserialize, Serialize,
    },
    State,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm_rocket::Connection;

use crate::database::Db;
use crate::error::CustomError;
use crate::services::auth_service::{
    create_oauth_token, generate_token, get_token_user_id, is_valid_token, OauthTokenResult,
};
use crate::utils::BasicAuth;

const ALLOWED_SCOPES: &'static [&'static str] = &["identity"];

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(untagged)]
pub enum AuthorizeResult {
    AuthorizationCode(AuthorizationCodeResult),
    Token(OauthTokenResult),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorizationCodeResult {
    authorization_code: String,
}

/// The struct to be stored in redis between sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorizeCode {
    pub user: i64,
    pub scope: String,
    pub state: Option<String>,
    pub application_id: i64,
    pub redirect_uri: String,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum ResponseType {
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "token")]
    Token,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorizeData {
    scope: String,
    state: Option<String>,
    client_id: i64,
    redirect_uri: String,
    response_type: ResponseType,
}

#[derive(Debug, FromForm)]
pub struct TokenData {
    code: String,
    state: Option<String>,
    client_id: Option<i64>,
    client_secret: Option<String>,
    redirect_uri: String,
}

#[post("/api/oauth2/authorize", data = "<body>")]
pub async fn authorize(
    body: Json<AuthorizeData>,
    db: Connection<'_, Db>,
    redis: &State<redis::Client>,
    cookies: &CookieJar<'_>,
) -> Result<Json<AuthorizeResult>, CustomError> {
    let db = db.into_inner();
    let mut redis = redis.get_tokio_connection().await?;

    let token = match cookies.get("token") {
        Some(cookie) => cookie.value(),
        None => "",
    };

    if !body
        .scope
        .split_ascii_whitespace()
        .all(|scope| ALLOWED_SCOPES.iter().any(|allowed| &scope == allowed))
    {
        return Err(CustomError::Custom(
            Status::BadRequest,
            "Invalid scopes".into(),
        ));
    }

    if !is_valid_token(token.to_string(), &[], true, db).await? {
        return Err(CustomError::Custom(
            Status::Unauthorized,
            "Invalid token or insufficient scope".into(),
        ));
    }

    let application = application::Entity::find_by_id(body.client_id)
        .one(db)
        .await?;

    if application.is_none() {
        return Err(CustomError::Custom(
            Status::NotFound,
            "Invalid application".into(),
        ));
    }

    let user_id = get_token_user_id(token.to_string(), db).await?;

    let result = match body.response_type {
        ResponseType::Code => {
            _create_authorization_code(
                user_id,
                body.client_id,
                body.scope.clone(),
                body.state.clone(),
                body.redirect_uri.clone(),
                &mut redis,
            )
            .await?
        }
        ResponseType::Token => {
            let access_token = generate_token(256);
            AuthorizeResult::Token(
                create_oauth_token(
                    access_token,
                    None,
                    user_id,
                    body.client_id,
                    body.scope.clone(),
                    db,
                )
                .await?,
            )
        }
    };

    Ok(Json(result))
}

#[post("/api/oauth2/token", data = "<body>")]
pub async fn token(
    body: Form<TokenData>,
    db: Connection<'_, Db>,
    auth: BasicAuth,
    redis: &State<redis::Client>,
) -> Result<Json<OauthTokenResult>, CustomError> {
    let db = db.into_inner();
    let mut redis = redis.get_tokio_connection().await?;

    let code: Option<String> = redis
        .get(format!("authorization_codes:{}", body.code))
        .await?;

    if code.is_none() {
        return Err(CustomError::Custom(
            Status::BadRequest,
            "Invalid code".into(),
        ));
    }
    let code =
        serde_json::from_str::<AuthorizeCode>(&code.unwrap()).map_err(|_| CustomError::Simple)?;

    if body.state != code.state {
        return Err(CustomError::Custom(
            Status::BadRequest,
            "Invalid state".into(),
        ));
    }

    if body.redirect_uri != code.redirect_uri {
        return Err(CustomError::Custom(
            Status::BadRequest,
            "Invalid redirect uri".into(),
        ));
    }

    let client_id = match body.client_id {
        Some(c) => c,
        None => match auth.get_username() {
            Some(u) => u
                .parse::<i64>()
                .map_err(|_| CustomError::Custom(Status::BadRequest, "Invalid client_id".into()))?,
            None => {
                return Err(CustomError::Custom(
                    Status::BadRequest,
                    "Missing client_id".into(),
                ))
            }
        },
    };

    let client_secret = match body.client_secret.clone() {
        Some(c) => c,
        None => match auth.get_password() {
            Some(u) => u,
            None => {
                return Err(CustomError::Custom(
                    Status::BadRequest,
                    "Missing client_id".into(),
                ))
            }
        },
    };

    let app = application::Entity::find_by_id(client_id)
        .filter(application::Column::Secret.eq(client_secret))
        .one(db)
        .await?;

    if client_id != code.application_id || app.is_none() {
        return Err(CustomError::Custom(
            Status::BadRequest,
            "Incorrect application id or secret".into(),
        ));
    }

    let _: () = redis
        .del(format!("authorization_codes:{}", body.code))
        .await?;

    let access_token = generate_token(256);
    let refresh_token = generate_token(256);

    create_oauth_token(
        access_token,
        Some(refresh_token),
        code.user,
        code.application_id,
        code.scope,
        db,
    )
    .await
    .map(|m| Json(m))
}

async fn _create_authorization_code(
    user_id: i64,
    application_id: i64,
    scope: String,
    state: Option<String>,
    redirect_uri: String,
    redis: &mut redis::aio::Connection,
) -> Result<AuthorizeResult, CustomError> {
    let mut rng = OsRng::default();
    let mut result = [0; 32];
    rng.fill_bytes(&mut result);
    let authorization_code = base64::engine::general_purpose::STANDARD.encode(result);

    let code = AuthorizeCode {
        user: user_id,
        scope: scope,
        state: state,
        application_id: application_id,
        redirect_uri: redirect_uri,
    };

    let _: () = redis
        .set_ex(
            format!("authorization_codes:{authorization_code}"),
            serde_json::to_string(&code).map_err(|_| CustomError::Simple)?,
            600,
        )
        .await?;

    Ok(AuthorizeResult::AuthorizationCode(
        AuthorizationCodeResult { authorization_code },
    ))
}
