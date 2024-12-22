use crate::domain::{task, todo};
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::task::response::Response;
use crate::http::api::{FromModel, UuidParam};
use crate::infrastructure::database::Database;
use chrono::Local;
use rocket::post;
use rocket::serde::json::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, TransactionTrait};

#[post("/<household_id>/task/<task_id>/complete")]
pub async fn complete(
    db: &Database,
    user: LoggedInUser,
    household_id: UuidParam,
    task_id: UuidParam,
) -> Result<Json<Response>, ApiError> {
    user.in_household(db, *household_id).await?;

    let todo = todo::find_latest(db, *task_id)
        .await
        .map_err(ApiError::from)?
        .ok_or(ApiError::NotFound(()))?;

    let task = db
        .conn()
        .transaction(|tx| {
            Box::pin(async move {
                let mut todo = todo.into_active_model();

                todo.completed_by = Set(Some(user.id));
                todo.completed_on = Set(Some(Local::now().naive_local()));

                let todo = todo.save(tx).await?;

                let task = task::Entity::find_by_id(*task_id)
                    .one(tx)
                    .await?
                    .expect("We already found its Todo, so the task should exist.");

                todo::ActiveModel {
                    task_id: Set(*task_id),
                    iteration: Set(todo.iteration.unwrap() + 1),
                    due_date: Set(task
                        .recurrence_unit
                        .next(Local::now().date_naive(), task.recurrence_interval as u32)),
                    completed_by: Default::default(),
                    completed_on: Default::default(),
                }
                .insert(tx)
                .await?;

                Ok(task)
            })
        })
        .await
        .map_err(ApiError::from)?;

    Response::from_model(db, task)
        .await
        .map_err(ApiError::from)
        .map(Json::from)
}
