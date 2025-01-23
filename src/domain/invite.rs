use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "invites")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub secret_digest: String,
    pub household_id: Uuid,
    pub created_by: Uuid,
    pub valid_until: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Household,
    CreatedBy,
    JoinedUsers,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Household => Entity::has_one(super::household::Entity).into(),
            Self::CreatedBy => Entity::has_one(super::user::Entity).into(),
            Self::JoinedUsers => Entity::belongs_to(super::household_member::Entity)
                .from(Column::Id)
                .to(super::household_member::Column::JoinedViaInvite)
                .into(),
        }
    }
}

impl Related<super::household::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Household.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CreatedBy.def()
    }
}

impl Related<super::household_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JoinedUsers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
