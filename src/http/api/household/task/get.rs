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
