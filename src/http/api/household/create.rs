use crate::domain;
use crate::http::api::household::response::Response;
use crate::http::error::database_error::DatabaseError;
use crate::infrastructure::database::Database;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use crate::http::api::FromModel;
use crate::http::api::guards::logged_in_user::LoggedInUser;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Request {
    pub name: String,
}

#[post("/", data = "<request>")]
pub async fn create(
    user: LoggedInUser,
    request: Json<Request>,
    db: &Database,
) -> Result<Json<Response>, DatabaseError> {
    let household = domain::household::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(request.name.clone()),
    }
    .insert(db.conn())
    .await
    .map_err(DatabaseError::from)?;
    domain::household_member::ActiveModel {
        user_id: Set(user.id),
        household_id: Set(household.id),
    }
    .insert(db.conn())
    .await
    .map_err(DatabaseError::from)?;
    Response::from_model(db, household)
        .await
        .map(Json::from)
        .map_err(DatabaseError::from)
}
