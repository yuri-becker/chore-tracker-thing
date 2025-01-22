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
    pub joined_via_invite: Option<Uuid>
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Household,
    JoinedViaInvite
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::has_one(super::user::Entity).into(),
            Relation::Household => Entity::has_one(super::household::Entity).into(),
            Relation::JoinedViaInvite => Entity::has_one(super::invite::Entity).into()
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

impl Related<super::invite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JoinedViaInvite.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
