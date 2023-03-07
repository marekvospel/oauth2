use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use time::Time;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "access_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub token: String,
    #[sea_orm(unique)]
    pub refresh: Option<String>,

    pub owner: i64,
    pub expire: Time,
    pub client_id: Option<i64>,
    pub scope: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
