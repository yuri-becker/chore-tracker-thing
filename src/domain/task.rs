use crate::domain::recurrence_unit::RecurrenceUnit;
use crate::infrastructure::database::Database;
use chrono::NaiveDate;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub household_id: Uuid,
    pub title: String,
    pub recurrence_unit: RecurrenceUnit,
    pub recurrence_interval: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Household,
    Todos,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Household => Entity::has_one(super::household::Entity).into(),
            Self::Todos => Entity::belongs_to(super::todo::Entity)
                .from(Column::Id)
                .to(super::todo::Column::TaskId)
                .into(),
        }
    }
}

impl Related<super::household::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Household.def()
    }
}

impl Related<super::todo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Todos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn latest_todo(&self, db: &Database) -> Result<Option<super::todo::Model>, DbErr> {
        super::todo::Entity::find_latest(db, self.id).await
    }

    pub fn next_recurrence(&self) -> NaiveDate {
        self.recurrence_unit
            .next_now(self.recurrence_interval as u32)
    }
}
