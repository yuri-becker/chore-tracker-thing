use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[serde(crate = "rocket::serde")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "recurrence_unit")]
pub enum RecurrenceUnit {
    #[sea_orm(string_value = "Days")]
    Days,
    #[sea_orm(string_value = "Weeks")]
    Weeks,
    #[sea_orm(string_value = "Months")]
    Month,
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
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Household => Entity::has_one(super::household::Entity).into(),
        }
    }
}

impl Related<super::household::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Household.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
