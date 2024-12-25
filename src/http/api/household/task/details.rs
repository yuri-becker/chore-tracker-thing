use crate::domain::task::RecurrenceUnit;
use crate::domain::{task, todo};
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::UuidParam;
use crate::infrastructure::database::Database;
use chrono::{NaiveDate, NaiveDateTime};
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::EntityTrait;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PastCompletion {
    pub iteration: i32,
    pub due_on: NaiveDate,
    pub completed_on: NaiveDateTime,
    pub completed_by: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TaskDetails {
    pub id: Uuid,
    pub title: String,
    pub recurrence_unit: RecurrenceUnit,
    pub recurrent_interval: u16,
    pub next_due: Option<NaiveDate>,
    pub past_completions: Vec<PastCompletion>,
}

#[get("/<household_id>/task/<task_id>")]
pub async fn details(
    db: &Database,
    user: LoggedInUser,
    household_id: UuidParam,
    task_id: UuidParam,
) -> Result<Json<TaskDetails>, ApiError> {
    user.in_household(db, *household_id).await?;
    let task = task::Entity::find_by_id(*task_id)
        .find_with_related(todo::Entity)
        .all(db.conn())
        .await
        .map_err(ApiError::from)?;
    let (task, todos) = task.first().ok_or(ApiError::NotFound(()))?;

    let task = task.to_owned();
    let (completions, uncompleted): (Vec<_>, Vec<_>) = todos
        .iter()
        .map(|it| it.to_owned())
        .partition(|it| it.completed_on.is_some());
    let uncompleted = uncompleted.first();

    Ok(Json::from(TaskDetails {
        id: task.id,
        title: task.title,
        recurrence_unit: task.recurrence_unit,
        recurrent_interval: task
            .recurrence_interval
            .try_into()
            .expect("recurrence_interval below 0 is prevented by database checks."),
        next_due: uncompleted.map(|it| it.due_date),
        past_completions: completions.iter().map(|completion| PastCompletion {
            iteration: completion.iteration,
            due_on: completion.due_date,
            completed_on: completion.completed_on
                .expect("This vector only containing todos with completion_on being Some, should have been checked before."),
            completed_by: completion.completed_by,
        }).collect(),
    }))
}
