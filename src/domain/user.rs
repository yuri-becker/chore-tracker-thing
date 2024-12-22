use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub display_name: Option<String>,
}

impl Model {
    pub fn name(&self) -> String {
        self.display_name.clone().unwrap_or(self.id.to_string())
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    HouseholdMemberships,
    TodoCompletions,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::HouseholdMemberships => Entity::belongs_to(super::household_member::Entity)
                .from(Column::Id)
                .to(super::household_member::Column::UserId)
                .into(),
            Self::TodoCompletions => Entity::belongs_to(super::todo::Entity)
                .from(Column::Id)
                .to(super::todo::Column::CompletedBy)
                .into(),
        }
    }
}

impl Related<super::household_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HouseholdMemberships.def()
    }
}

impl Related<super::todo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TodoCompletions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::now_v7()),
            display_name: NotSet,
        }
    }
}
