use crate::domain;
use crate::infrastructure::database::Database;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;

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
    Invites,
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
            Self::Invites => Entity::belongs_to(super::invite::Entity)
                .from(Column::Id)
                .to(super::invite::Column::HouseholdId)
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

impl Related<super::invite::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invites.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn create(db: &Database, name: String, user: Uuid) -> Result<Model, DbErr> {
    let household = ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(name),
    }
    .insert(db.conn())
    .await?;
    domain::household_member::ActiveModel {
        user_id: Set(user),
        household_id: Set(household.id),
        joined_via_invite: NotSet,
    }
    .insert(db.conn())
    .await?;
    Ok(household)
}
