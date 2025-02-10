use crate::domain::recurrence_unit::RecurrenceUnit;
use crate::domain::task;
use crate::http::api::FromModel;
use crate::infrastructure::database::Database;
use chrono::NaiveDate;
use rocket::async_trait;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::DbErr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub recurrence_unit: RecurrenceUnit,
    pub recurrent_interval: u16,
    pub next_due: Option<NaiveDate>,
}

#[async_trait]
impl FromModel<task::Model> for Task {
    async fn from_model(db: &Database, value: task::Model) -> Result<Self, DbErr> {
        let next_due = value.latest_todo(db).await?.map(|it| it.due_date);

        Ok(Self {
            id: value.id,
            title: value.title,
            recurrence_unit: value.recurrence_unit,
            recurrent_interval: value.recurrence_interval.try_into().expect(
                "recurrence_interval below 0 should be prevented by database CHECK constraint.",
            ),
            next_due,
        })
    }
}
