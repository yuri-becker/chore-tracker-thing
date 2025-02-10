use crate::infrastructure::database::Database;
use chrono::{Local, Months, NaiveDate, TimeDelta};
use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use std::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(crate = "rocket::serde")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "recurrence_unit")]
pub enum RecurrenceUnit {
    #[sea_orm(string_value = "Days")]
    Days,
    #[sea_orm(string_value = "Weeks")]
    Weeks,
    #[sea_orm(string_value = "Month")]
    Months,
}

impl RecurrenceUnit {
    pub fn next(&self, naive_date: NaiveDate, interval: u32) -> NaiveDate {
        match self {
            RecurrenceUnit::Days => naive_date.add(TimeDelta::days(interval as i64)),
            RecurrenceUnit::Weeks => naive_date.add(TimeDelta::weeks(interval as i64)),
            RecurrenceUnit::Months => naive_date.add(Months::new(interval)),
        }
    }

    pub fn next_now(&self, interval: u32) -> NaiveDate {
        self.next(Local::now().date_naive(), interval)
    }
}

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
