use crate::domain::task;
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::task::response::Response;
use crate::http::api::{FromModel, UuidParam};
use crate::infrastructure::database::Database;
use rocket::futures::future::try_join_all;
use rocket::get;
use rocket::serde::json::Json;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};

#[get("/<household_id>/task")]
pub async fn get(
    db: &Database,
    user: LoggedInUser,
    household_id: UuidParam,
) -> Result<Json<Vec<Response>>, ApiError> {
    user.in_household(db, *household_id).await?;
    let tasks = task::Entity::find()
        .filter(task::Column::HouseholdId.eq(*household_id))
        .all(db.conn())
        .await
        .map_err(ApiError::from)?;

    let tasks = tasks
        .iter()
        .map(|task| Response::from_model(db, task.to_owned()))
        .collect::<Vec<_>>();
    let tasks = try_join_all(tasks).await.map_err(ApiError::from)?;
    Ok(Json(tasks))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::task::RecurrenceUnit;
    use crate::test_environment::{TestEnvironment, TestUser};
    use rocket::http::Status;
    use rocket::{async_test, routes};
    use sea_orm::ActiveModelTrait;
    use sea_orm::ActiveValue::Set;
    use uuid::Uuid;

    #[async_test]
    async fn test_get_tasks() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![get])
            .launch()
            .await;
        let household_1 = env.create_household(None, TestUser::A).await;
        let household_2 = env.create_household(None, TestUser::A).await;

        task::ActiveModel {
            id: Set(Uuid::now_v7()),
            title: Set("Clean Windows".to_string()),
            household_id: Set(household_1.id),
            recurrence_unit: Set(RecurrenceUnit::Months),
            recurrence_interval: Set(2),
        }
        .insert(env.database().conn())
        .await
        .unwrap();

        task::ActiveModel {
            id: Set(Uuid::now_v7()),
            title: Set("Vacuum".to_string()),
            household_id: Set(household_2.id),
            recurrence_unit: Set(RecurrenceUnit::Days),
            recurrence_interval: Set(2),
        }
        .insert(env.database().conn())
        .await
        .unwrap();

        let response = env
            .get(format!("/{}/task", household_1.id))
            .header(env.header_user_a())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let response: Vec<Response> = response.into_json().await.unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response.first().unwrap().title, "Clean Windows".to_string());
    }
}
