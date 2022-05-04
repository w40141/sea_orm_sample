//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub profile: Option<String>,
    pub enable: i8,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::favorite::Entity")]
    Favorite,
    #[sea_orm(has_many = "super::post::Entity")]
    Post,
}

impl Related<super::favorite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Favorite.def()
    }
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
