use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;

use crate::{database::Db, Token};
use ::entity::token;

#[get("/me")]
pub async fn me(db: Connection<'_, Db>, token: Token) -> () {
    let db = db.into_inner();
    let req_token = token.0;

    let _token = token::Entity::find_by_id(&req_token).one(db).await;

    println!("{}", req_token)
}
