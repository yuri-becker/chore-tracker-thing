use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    HouseholdMemberships,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::HouseholdMemberships => Entity::belongs_to(super::household_member::Entity)
                .from(Column::Id)
                .to(super::household_member::Column::UserId)
                .into(),
        }
    }
}

impl Related<super::household_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HouseholdMemberships.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
        }
    }
}
