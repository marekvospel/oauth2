use base64::Engine;
use rand::{rngs::OsRng, RngCore};
use redis::AsyncCommands;
use rocket::{
    serde::{
        json::{serde_json, Json},
        Deserialize, Serialize,
    },
    State,
};
use sea_orm_rocket::Connection;

use crate::{
    database::Db,
    utils::auth::{get_token_user_id, is_valid_token},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorizeData {
    scopes: String,
    token: String,
    state: Option<String>,
}

#[post("/api/internal/authorize_code", data = "<body>")]
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
        scopes: body.scopes.clone(),
        state: body.state.clone(),
    };

    let _: () = redis
        .set_ex(
            format!("authorization_codes:{authorization_code}"),
            serde_json::to_string(&code).unwrap(),
            600000,
        )
        .await
        .unwrap();

    authorization_code
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthorizeCode {
    pub user: i64,
    pub scopes: String,
    pub state: Option<String>,
}
