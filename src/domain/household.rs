use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "households")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Members,
    Tasks,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Members => Entity::belongs_to(super::household_member::Entity)
                .from(Column::Id)
                .to(super::household_member::Column::HouseholdId)
                .into(),
            Self::Tasks => Entity::belongs_to(super::task::Entity)
                .from(Column::Id)
                .to(super::task::Column::HouseholdId)
                .into(),
        }
    }
}

impl Related<super::household_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tasks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
