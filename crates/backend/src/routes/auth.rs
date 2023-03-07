use crate::database::Db;
use ::entity::user;
use rocket::serde::{json::Json, Deserialize, Serialize};
use sea_orm::*;
use sea_orm_rocket::Connection;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginData {
    email: String,
    password: String,
}

#[post("/auth/login", data = "<input>")]
pub async fn login(db: Connection<'_, Db>, input: Json<LoginData>) {
    let db = db.into_inner();

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(input.email.clone()))
        .one(db)
        .await;

    if user.is_err() {
        return;
    }
    let user = user.unwrap();
    if user.is_none() {
        return;
    }
    let user = user.unwrap();

    println!("{user:?}");

    let correct_password = bcrypt::verify(&input.password, &user.password);

    println!("{correct_password:?}");
}
