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

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_environment::{TestEnvironment, TestUser};
    use chrono::{Days, Local};
    use rocket::http::Status;
    use rocket::{async_test, routes};
    use sea_orm::ActiveValue::Set;
    use sea_orm::{ActiveModelTrait, NotSet};

    #[async_test]
    async fn test_throws_404_if_not_exists() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![details])
            .launch()
            .await;

        let household = env.create_household(None, TestUser::A).await;

        let response = env
            .get(format!("/{}/task/{}", household.id, Uuid::now_v7()))
            .header(env.header_user_a())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::NotFound);
    }

    #[async_test]
    async fn test_get_details() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![details])
            .launch()
            .await;
        let household = env.create_household(None, TestUser::A).await;
        let task = task::ActiveModel {
            id: Set(Uuid::now_v7()),
            title: Set("Vacuum".to_string()),
            recurrence_interval: Set(2),
            recurrence_unit: Set(RecurrenceUnit::Days),
            household_id: Set(household.id),
        }
        .insert(env.database().conn())
        .await
        .unwrap();

        todo::ActiveModel {
            task_id: Set(task.id),
            iteration: Set(0),
            completed_by: Set(Some(env.user_a)),
            due_date: Set(Local::now()
                .naive_local()
                .date()
                .checked_sub_days(Days::new(7))
                .unwrap()),
            completed_on: Set(Some(Local::now().naive_local())),
        }
        .insert(env.database().conn())
        .await
        .unwrap();

        todo::ActiveModel {
            task_id: Set(task.id),
            iteration: Set(1),
            due_date: Set(Local::now()
                .naive_local()
                .date()
                .checked_add_days(Days::new(7))
                .unwrap()),
            completed_on: NotSet,
            completed_by: NotSet,
        }
        .insert(env.database().conn())
        .await
        .unwrap();

        let response = env
            .get(format!("/{}/task/{}", household.id, task.id))
            .header(env.header_user_a())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let response: TaskDetails = response.into_json().await.unwrap();
        assert_eq!(response.title, "Vacuum");
        assert_eq!(response.past_completions.len(), 1);
        assert_eq!(
            response.next_due.unwrap(),
            Local::now()
                .naive_local()
                .date()
                .checked_add_days(Days::new(7))
                .unwrap()
        );
        let completion = response.past_completions.first().unwrap();
        assert_eq!(completion.completed_by, Some(env.user_a));
        assert_eq!(completion.iteration, 0);
    }
}
