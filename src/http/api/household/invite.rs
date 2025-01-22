use crate::domain::invite_secret::InviteSecret;
use crate::domain::invite;
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::UuidParam;
use crate::infrastructure::database::Database;
use chrono::{DateTime, TimeDelta, Utc};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{error, get};
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveModelTrait;
use std::ops::Add;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub invite_code: Uuid,
    pub secret: String,
    pub valid_until: DateTime<Utc>,
}

#[get("/<household_id>/invite")]
pub async fn generate_invite(
    db: &Database,
    user: LoggedInUser,
    household_id: UuidParam,
) -> Result<Json<Response>, ApiError> {
    user.in_household(db, *household_id).await?;

    let id = Uuid::now_v7();
    let secret = InviteSecret::generate().map_err(|err| {
        error!("Failed to generate invite secret: {:?}", err);
        ApiError::InternalServerError(())
    })?;

    let valid_until = Utc::now().add(TimeDelta::hours(24));
    invite::ActiveModel {
        id: Set(id),
        household_id: Set(*household_id),
        created_by: Set(user.id),
        secret_digest: Set(secret.digest),
        valid_until: Set(valid_until.clone().naive_local()),
    }
    .insert(db.conn())
    .await
    .map_err(ApiError::from)?;

    Ok(Json(Response {
        invite_code: id,
        secret: secret.secret,
        valid_until,
    }))
}

