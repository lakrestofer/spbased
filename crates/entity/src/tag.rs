//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub content: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tag_map::Entity")]
    TagMap,
}

impl Related<super::tag_map::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TagMap.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
