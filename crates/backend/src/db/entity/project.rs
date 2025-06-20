//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::group::Entity")]
    Group,
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<super::user_group_project::Entity> for Entity {
    fn to() -> RelationDef {
        super::group::Relation::UserGroupProject.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::group::Relation::Project.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
