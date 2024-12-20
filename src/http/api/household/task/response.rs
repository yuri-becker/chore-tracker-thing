use crate::domain::task;
use crate::domain::task::RecurrenceUnit;
use crate::http::api::FromModel;
use crate::infrastructure::database::Database;
use rocket::async_trait;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::DbErr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub id: Uuid,
    pub title: String,
    pub recurrence_unit: RecurrenceUnit,
    pub recurrent_interval: u16,
}

#[async_trait]
impl FromModel<task::Model> for Response {
    async fn from_model(_: &Database, value: task::Model) -> Result<Self, DbErr> {
        Ok(Self {
            id: value.id,
            title: value.title,
            recurrence_unit: value.recurrence_unit,
            recurrent_interval: value.recurrence_interval.try_into().expect(
                "recurrence_interval below 0 should be prevented by database CHECK constraint.",
            ),
        })
    }
}
