use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "access_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub token: String,
    #[sea_orm(unique)]
    pub refresh: Option<String>,

    pub owner: i64,
    pub expire: OffsetDateTime,
    pub client_id: Option<i64>,
    pub scope: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Owner",
        to = "super::user::Column::Id"
    )]
    Owner,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
