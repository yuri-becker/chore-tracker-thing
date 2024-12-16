use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "household_members")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: Uuid,
    #[sea_orm(primary_key)]
    pub household_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Household,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::has_one(super::user::Entity).into(),
            Relation::Household => Entity::has_one(super::household::Entity).into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::household::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Household.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
