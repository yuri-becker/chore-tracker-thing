use crate::domain::task;
use crate::domain::task::RecurrenceUnit;
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::task::response::Response;
use crate::http::api::{FromModel, UuidParam};
use crate::infrastructure::database::Database;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
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
) -> Result<Json<Response>, ApiError> {
    user.in_household(db, *household_id).await?;
    let request = request.0;
    if request.recurrence_interval < 1 {
        return Err(ApiError::InvalidRequest(
            "recurrence_interval needs to be at least 1.",
        ));
    }
    let task = task::ActiveModel {
        id: Set(Uuid::now_v7()),
        household_id: Set(*household_id),
        title: Set(request.title),
        recurrence_unit: Set(request.recurrence_unit),
        recurrence_interval: Set(i32::from(request.recurrence_interval)),
    }
    .insert(db.conn())
    .await
    .map_err(ApiError::from)?;

    Response::from_model(db, task)
        .await
        .map_err(ApiError::from)
        .map(Json::from)
}
