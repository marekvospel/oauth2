use std::{io::Cursor, ops::Add};

use ::entity::token::{self as Token};
use base64::Engine;
use rand::{rngs::OsRng, RngCore};
use redis::AsyncCommands;
use rocket::response::Responder;
use rocket::{
    form::{Form, FromForm},
    http::{ContentType, Status},
    serde::{
        json::{serde_json, Json},
        Deserialize, Serialize,
    },
    Request, Response, State,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use sea_orm_rocket::Connection;
use time::{Duration, OffsetDateTime};

use crate::{
    database::Db,
    utils::auth::{generate_token, get_token_user_id, is_valid_token},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorizeData {
    scopes: String,
    token: String,
    state: Option<String>,
}

#[post("/api/internal/oauth2/authorize_code", data = "<body>")]
pub async fn authorize(
    body: Json<AuthorizeData>,
    db: Connection<'_, Db>,
    redis: &State<redis::Client>,
) -> String {
    let db = db.into_inner();
    let mut redis = redis.get_tokio_connection().await.unwrap();

    if !is_valid_token(body.token.clone(), &["me".into()], db)
        .await
        .unwrap_or(false)
    {
        return "Internal error".into();
    }

    let user_id = get_token_user_id(body.token.clone(), db).await.unwrap();
    let mut rng = OsRng::default();
    let mut result = [0; 32];
    rng.fill_bytes(&mut result);
    let authorization_code = base64::engine::general_purpose::STANDARD.encode(result);

    let code = AuthorizeCode {
        user: user_id,
        scope: body.scopes.clone(),
        state: body.state.clone(),
    };

    let _: () = redis
        .set_ex(
            format!("authorization_codes:{authorization_code}"),
            serde_json::to_string(&code).unwrap(),
            600,
        )
        .await
        .unwrap();

    authorization_code
}

#[derive(Debug, FromForm)]
pub struct TokenData {
    code: String,
    state: Option<String>,
}

#[post("/api/oauth2/token", data = "<body>")]
pub async fn token(
    body: Form<TokenData>,
    db: Connection<'_, Db>,
    redis: &State<redis::Client>,
) -> Result<OauthTokenSuccess, String> {
    let db = db.into_inner();
    let mut redis = redis.get_tokio_connection().await.unwrap();

    let code: Option<String> = redis
        .get(format!("authorization_codes:{}", body.code))
        .await
        .unwrap();

    if code.is_none() {
        return Err("Invalid code".into());
    }
    let code = serde_json::from_str::<AuthorizeCode>(&code.unwrap()).unwrap();

    if body.state != code.state {
        return Err("Invalid state".into());
    }

    let _: () = redis
        .del(format!("authorization_codes:{}", body.code))
        .await
        .unwrap();

    let access_token = generate_token();
    let refresh_token = generate_token();

    Token::Entity::delete_many()
        .filter(Token::Column::Owner.eq(code.user))
        .filter(Token::Column::ClientId.is_null())
        .exec(db)
        .await
        .unwrap();

    Token::ActiveModel {
        token: ActiveValue::Set(access_token.clone()),
        refresh: ActiveValue::Set(Some(refresh_token.clone())),
        // This token is not meant to be used in the Authorization header, but in
        token_type: ActiveValue::Set("Bearer".into()),
        owner: ActiveValue::Set(code.user),
        expire: ActiveValue::Set(OffsetDateTime::now_utc().add(Duration::days(7))),
        client_id: ActiveValue::Set(None),
        scope: ActiveValue::Set(code.scope.clone()),
    }
    .insert(db)
    .await
    .unwrap();

    Ok(OauthTokenSuccess {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: Duration::days(7).whole_seconds(),
        scope: code.scope,
    })
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OauthTokenSuccess {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for OauthTokenSuccess {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorizeCode {
    pub user: i64,
    pub scope: String,
    pub state: Option<String>,
}
