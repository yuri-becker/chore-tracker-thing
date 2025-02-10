use crate::domain::task::RecurrenceUnit;
use crate::domain::{task, todo};
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::task::response::Task;
use crate::http::api::{FromModel, UuidParam};
use crate::infrastructure::database::Database;
use chrono::Local;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, TransactionTrait};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Request {
    pub title: String,
    pub recurrence_unit: RecurrenceUnit,
    pub recurrence_interval: u16,
}

#[post("/<household_id>/task", data = "<request>")]
pub async fn create(
    db: &Database,
    user: LoggedInUser,
    household_id: UuidParam,
    request: Json<Request>,
) -> Result<Json<Task>, ApiError> {
    user.in_household(db, *household_id).await?;
    let request = request.0;
    if request.recurrence_interval < 1 {
        return Err(ApiError::InvalidRequest(
            "recurrence_interval needs to be at least 1.",
        ));
    }
    let task = db
        .conn()
        .transaction(|tx| {
            Box::pin(async move {
                let task = task::ActiveModel {
                    id: Set(Uuid::now_v7()),
                    household_id: Set(*household_id),
                    title: Set(request.title),
                    recurrence_unit: Set(request.recurrence_unit),
                    recurrence_interval: Set(i32::from(request.recurrence_interval)),
                }
                .insert(tx)
                .await?;
                todo::ActiveModel::initial(task.id, Local::now().date_naive())
                    .insert(tx)
                    .await?;
                Ok(task)
            })
        })
        .await
        .map_err(ApiError::from)?;

    Task::from_model(db, task)
        .await
        .map(Json::from)
        .map_err(ApiError::from)
}

#[cfg(test)]
mod test {
    use crate::domain::task::RecurrenceUnit;
    use crate::domain::{task, todo};
    use crate::test_environment::{TestEnvironment, TestUser};
    use chrono::Local;
    use rocket::http::Status;
    use rocket::serde::json::serde_json::json;
    use rocket::{async_test, routes};
    use sea_orm::EntityTrait;

    struct Helpers {}

    impl Helpers {
        pub async fn create_env() -> TestEnvironment {
            TestEnvironment::builder()
                .await
                .mount(routes![super::create])
                .launch()
                .await
        }
    }

    #[async_test]
    async fn test_throws_422_when_recurrence_interval_is_zero() {
        let env = Helpers::create_env().await;
        let household = env.create_household(None, TestUser::A).await;
        let response = env
            .post(format!("/{}/task", household.id))
            .header(env.header_user_a())
            .json(&json!({
                "title": "Vacuum",
                "recurrenceUnit": "Weeks",
                "recurrenceInterval": 0
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[async_test]
    async fn test_create_task() {
        let env = Helpers::create_env().await;
        let household = env.create_household(None, TestUser::A).await;
        let response = env
            .post(format!("/{}/task", household.id))
            .header(env.header_user_a())
            .json(&json!({
                "title": "Vacuum",
                "recurrenceUnit": "Weeks",
                "recurrenceInterval": 1
            }))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let response: super::Task = response.into_json().await.unwrap();
        let task = task::Entity::find_by_id(response.id)
            .find_with_related(todo::Entity)
            .all(env.database().conn())
            .await
            .unwrap();
        assert_eq!(task.len(), 1);
        let (task, todo) = task.first().unwrap();
        assert_eq!(task.recurrence_unit, RecurrenceUnit::Weeks);
        assert_eq!(task.recurrence_interval, 1);
        assert_eq!(todo.len(), 1);
        let todo = todo.first().unwrap();
        assert!(todo.completed_on.is_none());
        assert!(todo.completed_by.is_none());
        assert_eq!(todo.due_date, Local::now().date_naive())
    }
}
