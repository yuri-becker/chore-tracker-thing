use crate::domain::task::RecurrenceUnit;
use crate::domain::{task, todo};
use crate::http::api::api_error::{ApiError, ApiResult};
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::task::response::Task;
use crate::http::api::{FromModel, UuidParam};
use crate::infrastructure::database::Database;
use rocket::put;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Request {
    title: Option<String>,
    recurrence_unit: Option<RecurrenceUnit>,
    recurrence_interval: Option<u16>,
    #[serde(default)]
    move_next_todo: bool,
}

impl Request {
    fn recurrence_changed(&self) -> bool {
        self.recurrence_unit.is_some() || self.recurrence_interval.is_some()
    }
}

#[put("/<household_id>/task/<task_id>", data = "<request>")]
pub async fn edit(
    db: &Database,
    user: LoggedInUser,
    household_id: UuidParam,
    task_id: UuidParam,
    request: Json<Request>,
) -> ApiResult<Task> {
    user.in_household(db, *household_id).await?;
    let mut task = task::Entity::find_by_id(*task_id)
        .one(db.conn())
        .await?
        .ok_or(ApiError::NotFound(()))?
        .into_active_model();

    if let Some(title) = request.title.as_ref() {
        task.title = Set(title.clone());
    };
    if let Some(recurrence_unit) = request.recurrence_unit.as_ref() {
        task.recurrence_unit = Set(recurrence_unit.clone());
    }
    if let Some(recurrence_interval) = request.recurrence_interval.as_ref() {
        task.recurrence_interval = Set((*recurrence_interval).into());
    }

    let task = task.update(db.conn()).await?;
    if request.recurrence_changed() && request.move_next_todo {
        let next_recurrence = task.next_recurrence();
        if let Some(latest_todo) = task.latest_todo(db).await? {
            let mut latest_todo = latest_todo.into_active_model();
            latest_todo.due_date = Set(next_recurrence);
            latest_todo.update(db.conn()).await?;
        } else {
            // Task has no latest todo – that is a case that should not exist, but we handle it here
            // for safety.
            todo::ActiveModel::initial(task.id, next_recurrence)
                .insert(db.conn())
                .await?;
        }
    }
    Ok(Json(Task::from_model(db, task).await?))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain;
    use crate::domain::{task, todo};
    use crate::http::api::household::task::response;
    use crate::test_environment::{TestEnvironment, TestUser};
    use chrono::{Days, Local};
    use rocket::http::Status;
    use rocket::serde::json::serde_json::json;
    use rocket::{async_test, routes};
    use sea_orm::ActiveValue::Set;
    use sea_orm::{ActiveModelTrait, EntityTrait};
    use std::ops::Add;
    use uuid::Uuid;

    #[async_test]
    async fn test_edit_nothing() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![super::edit])
            .launch()
            .await;
        let household = env.create_household(None, TestUser::A).await.id;
        let id = domain::task::ActiveModel {
            id: Set(Uuid::now_v7()),
            household_id: Set(household),
            title: Set("Vacuum".to_string()),
            recurrence_unit: Set(RecurrenceUnit::Weeks),
            recurrence_interval: Set(1),
        }
        .insert(env.database().conn())
        .await
        .unwrap()
        .id;

        let response = env
            .put(format!("/{household}/task/{id}"))
            .header(env.header_user_a())
            .json(&json!({}))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);

        let task = domain::task::Entity::find_by_id(id)
            .one(env.database().conn())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(task.title, "Vacuum");
        assert_eq!(task.recurrence_unit, RecurrenceUnit::Weeks);
        assert_eq!(task.recurrence_interval, 1);
    }

    #[async_test]
    async fn test_edit_name() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![super::edit])
            .launch()
            .await;
        let household = env.create_household(None, TestUser::A).await.id;
        let id = domain::task::ActiveModel {
            id: Set(Uuid::now_v7()),
            household_id: Set(household),
            title: Set("Dash Wishes".to_string()),
            recurrence_unit: Set(RecurrenceUnit::Days),
            recurrence_interval: Set(1),
        }
        .insert(env.database().conn())
        .await
        .unwrap()
        .id;

        let response = env
            .put(format!("/{household}/task/{id}"))
            .header(env.header_user_a())
            .json(&json!({
                "title": "Wash Dishes"
            }))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let response: response::Task = response.into_json().await.unwrap();
        assert_eq!(response.title, "Wash Dishes");
        let task = domain::task::Entity::find_by_id(id)
            .one(env.database().conn())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(task.title, "Wash Dishes");
    }

    #[async_test]
    async fn test_edit_recurrence_interval_no_move() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![super::edit])
            .launch()
            .await;
        let now = Local::now().date_naive();
        let household = env.create_household(None, TestUser::A).await.id;
        let id = domain::task::ActiveModel {
            id: Set(Uuid::now_v7()),
            household_id: Set(household),
            title: Set("Clean Shulk's Litter Box".to_string()),
            recurrence_unit: Set(RecurrenceUnit::Days),
            recurrence_interval: Set(1),
        }
        .insert(env.database().conn())
        .await
        .unwrap()
        .id;
        domain::todo::ActiveModel::initial(id, now.add(Days::new(1)))
            .insert(env.database().conn())
            .await
            .unwrap();

        let response = env
            .put(format!("/{household}/task/{id}"))
            .header(env.header_user_a())
            .json(&json!({
                "recurrenceInterval": 2
            }))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let response: response::Task = response.into_json().await.unwrap();
        assert_eq!(response.recurrent_interval, 2u16);
        let task = domain::task::Entity::find_by_id(id)
            .one(env.database().conn())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(task.recurrence_interval, 2);
        let todo = task.latest_todo(env.database()).await.unwrap().unwrap();
        assert_eq!(todo.due_date, now.add(Days::new(1)));
    }

    #[async_test]
    async fn test_edit_recurrence_unit_and_move_next_with_no_todo() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![super::edit])
            .launch()
            .await;
        let household = env.create_household(None, TestUser::A).await.id;
        let task = task::ActiveModel {
            id: Set(Uuid::now_v7()),
            household_id: Set(household),
            title: Set("Bring out the Trash".to_string()),
            recurrence_interval: Set(1),
            recurrence_unit: Set(RecurrenceUnit::Days),
        }
        .insert(env.database().conn())
        .await
        .unwrap()
        .id;
        let now = Local::now().date_naive();

        let response: response::Task = env
            .put(format!("/{household}/task/{task}"))
            .header(env.header_user_a())
            .json(&json!({
                "recurrenceUnit": "Weeks",
                "moveNextTodo": true
            }))
            .dispatch()
            .await
            .into_json()
            .await
            .unwrap();
        assert_eq!(response.recurrence_unit, RecurrenceUnit::Weeks);
        assert_eq!(response.next_due.unwrap(), now.add(Days::new(7)));
        let todo = task::Entity::find_by_id(task)
            .one(env.database().conn())
            .await
            .unwrap()
            .unwrap()
            .latest_todo(env.database())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(todo.due_date, now.add(Days::new(7)));
    }

    #[async_test]
    async fn test_edit_recurrence_interval_and_move_next_with_existing_todo() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![super::edit])
            .launch()
            .await;
        let household = env.create_household(None, TestUser::A).await.id;
        let task = task::ActiveModel {
            id: Set(Uuid::now_v7()),
            household_id: Set(household),
            title: Set("Clean Shower".to_string()),
            recurrence_interval: Set(2),
            recurrence_unit: Set(RecurrenceUnit::Days),
        }
        .insert(env.database().conn())
        .await
        .unwrap()
        .id;
        let now = Local::now().date_naive();
        todo::ActiveModel::initial(task, now.add(Days::new(3)))
            .insert(env.database().conn())
            .await
            .unwrap();

        let response = env
            .put(format!("/{household}/task/{task}"))
            .header(env.header_user_a())
            .json(&json!({
                "recurrenceUnit": "Weeks",
                "moveNextTodo": true
            }))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let response: response::Task = response.into_json().await.unwrap();
        assert_eq!(response.recurrence_unit, RecurrenceUnit::Weeks);
        assert_eq!(response.next_due.unwrap(), now.add(Days::new(14)));
        let todo = task::Entity::find_by_id(task)
            .one(env.database().conn())
            .await
            .unwrap()
            .unwrap()
            .latest_todo(env.database())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(todo.due_date, now.add(Days::new(14)));
    }
}
