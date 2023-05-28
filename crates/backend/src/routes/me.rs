use ::entity::user;
use rocket::{
    http::{CookieJar, Status},
    serde::{json::Json, Deserialize, Serialize},
};
use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;

use crate::{
    database::Db,
    error::CustomError,
    services::auth_service::{get_token_user_id, is_valid_token},
    utils::Authorization,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MeResult {
    id: i64,
    username: String,
    email: String,
}

#[get("/api/@me")]
pub async fn get_me(
    authorization: Authorization,
    cookies: &CookieJar<'_>,
    db: Connection<'_, Db>,
) -> Result<Json<MeResult>, CustomError> {
    let db = db.into_inner();
    println!("{:?}", authorization.0);

    let mut token = match authorization.0 {
        Some(t) => t,
        None => match cookies.get("token") {
            Some(t) => t.value().to_string(),
            None => {
                return Err(CustomError::Custom(
                    Status::Unauthorized,
                    "Missing authorization header".into(),
                ));
            }
        },
    };
    if token.len() >= 7 && &token[..7] == "Bearer " {
        token = (&token[7..]).to_string()
    }

    if !is_valid_token(token.clone(), &["identity".into()], true, db).await? {
        return Err(CustomError::Custom(
            Status::Unauthorized,
            "Invalid token or insufficient scope".into(),
        ));
    }

    let user_id = get_token_user_id(token, db).await?;

    let me = user::Entity::find_by_id(user_id).one(db).await?.unwrap();

    Ok(Json(MeResult {
        id: me.id,
        username: me.username,
        email: me.email,
    }))
}
