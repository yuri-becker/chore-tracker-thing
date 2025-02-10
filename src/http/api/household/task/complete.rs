use crate::domain::{task, todo};
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::task::response::Task;
use crate::http::api::{FromModel, UuidParam};
use crate::infrastructure::database::Database;
use chrono::Local;
use rocket::post;
use rocket::serde::json::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, NotSet, TransactionTrait};

#[post("/<household_id>/task/<task_id>/complete")]
pub async fn complete(
    db: &Database,
    user: LoggedInUser,
    household_id: UuidParam,
    task_id: UuidParam,
) -> Result<Json<Task>, ApiError> {
    user.in_household(db, *household_id).await?;

    let todo = todo::Entity::find_latest(db, *task_id)
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
                        .next_now(task.recurrence_interval as u32)),
                    completed_by: NotSet,
                    completed_on: NotSet,
                }
                .insert(tx)
                .await?;

                Ok(task)
            })
        })
        .await
        .map_err(ApiError::from)?;

    Task::from_model(db, task)
        .await
        .map_err(ApiError::from)
        .map(Json::from)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http::api::household::task::create;
    use crate::test_environment::{TestEnvironment, TestUser};
    use chrono::Days;
    use rocket::http::Status;
    use rocket::serde::json::serde_json::json;
    use rocket::{async_test, routes};
    use uuid::Uuid;

    #[async_test]
    async fn test_throws_404_if_not_exists() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![complete])
            .launch()
            .await;
        let household = env.create_household(None, TestUser::A).await;

        let response = env
            .post(format!(
                "/{}/task/{}/complete",
                household.id,
                Uuid::now_v7()
            ))
            .header(env.header_user_a())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::NotFound);
    }

    #[async_test]
    async fn test_complete_task() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![complete, create::create])
            .launch()
            .await;

        let household = env.create_household(None, TestUser::A).await;
        let task = env
            .post(format!("/{}/task", household.id))
            .header(env.header_user_a())
            .json(&json!({
                "title": "Vacuum",
                "recurrenceUnit": "Weeks",
                "recurrenceInterval": 1
            }))
            .dispatch()
            .await;
        let task: Task = task.into_json().await.unwrap();

        let complete = env
            .post(format!("/{}/task/{}/complete", household.id, task.id))
            .header(env.header_user_a())
            .dispatch()
            .await;
        assert_eq!(complete.status(), Status::Ok);
        let complete: Task = complete.into_json().await.unwrap();
        assert_eq!(complete.id, task.id);

        let old_todo = todo::Entity::find_by_id((task.id, 0))
            .one(env.database().conn())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(old_todo.completed_by, Some(env.user_a));
        assert!(old_todo.completed_on.is_some());

        let new_todo = todo::Entity::find_latest(env.database(), task.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            new_todo.due_date,
            task.next_due
                .unwrap()
                .checked_add_days(Days::new(7))
                .unwrap()
        );
        assert_eq!(new_todo.iteration, 1);
        assert_eq!(new_todo.completed_on, None);
        assert_eq!(new_todo.completed_by, None);
    }
}
