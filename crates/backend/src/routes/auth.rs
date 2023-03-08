use crate::database::Db;
use ::entity::access_token;
use ::entity::user;
use rocket::serde::{json::Json, Deserialize, Serialize};
use sea_orm::*;
use sea_orm_rocket::Connection;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginData {
    email: String,
    password: String,
}

#[post("/auth/login", data = "<input>")]
pub async fn login(db: Connection<'_, Db>, input: Json<LoginData>) -> &'static str {
    let db = db.into_inner();

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(input.email.clone()))
        .one(db)
        .await;

    if user.is_err() {
        return "Internal server error";
    }
    let user = user.unwrap();
    if user.is_none() {
        return "Username or password is invalid";
    }
    let user = user.unwrap();

    println!("{user:?}");

    let correct_password = bcrypt::verify(&input.password, &user.password);

    if correct_password.is_err() || !correct_password.unwrap() {}

    let token = access_token::ActiveModel {
        token: ActiveValue::Set("123".into()),
        refresh: ActiveValue::Set(None),
        owner: ActiveValue::Set(user.id),
        expire: ActiveValue::Set(OffsetDateTime::now_utc()),
        client_id: ActiveValue::Set(None),
        scope: ActiveValue::Set("".into()),
    };

    let success = token.insert(db).await;

    if success.is_ok() {
        "123"
    } else {
        "Internal server error"
    }
}
