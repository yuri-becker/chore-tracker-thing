use rocket::serde::{Deserialize, Serialize};
use rocket::post;
use rocket::serde::json::Json;
use uuid::Uuid;
use log::debug;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use domain::household;
use crate::domain;
use crate::domain::{household_member, invite};
use crate::domain::invite_secret::InviteSecret;
use crate::http::api::api_error::ApiError;
use crate::http::api::FromModel;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::response::Household;
use crate::infrastructure::database::Database;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Request {
    pub invite_code: String,
    pub secret: String,
}

#[post("/join", data = "<request>")]
pub async fn join(
    db: &Database,
    user: LoggedInUser,
    request: Json<Request>,
) -> Result<Json<Household>, ApiError> {
    let invite_code = Uuid::parse_str(&request.invite_code).map_err(|_| {
        debug!("invite_code is not a uuid");
        ApiError::NotFound(())
    })?;
    let invite = invite::Entity::find_by_id(invite_code)
        .one(db.conn())
        .await?;
    let invite = invite.ok_or(ApiError::NotFound(()))?;

    let valid = InviteSecret {
        secret: request.secret.clone(),
        digest: invite.secret_digest.clone()
    }.validate();
    if !valid {
        return Err(ApiError::DatabaseError(()));
    }

    let user_already_in_household = household_member::Entity::find()
        .filter(
            Condition::all()
                .add(household_member::Column::HouseholdId.eq(invite.household_id))
                .add(household_member::Column::UserId.eq(user.id))
        ).count(db.conn())
        .await? > 0;
    if user_already_in_household {
        return Err(ApiError::Conflict(()))
    }

    household_member::ActiveModel {
        user_id: Set(user.id),
        household_id: Set(invite.household_id),
        joined_via_invite: Set(Some(invite.id))
    }.insert(db.conn()).await?;

    let household = household::Entity::find_by_id(invite.household_id)
        .one(db.conn())
        .await?
        .expect("Household is ensured to exist by here.");

    let household = Household::from_model(db, household).await?;
    Ok(Json::from(household))
}