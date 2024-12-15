use crate::domain;
use crate::http::error::database_error::DatabaseError;
use crate::infrastructure::database::Database;
use crate::infrastructure::user::LoggedInUser;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{post, State};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Request {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Household {
    pub id: Uuid,
    pub name: String,
    pub members: Vec<Uuid>,
}

#[post("/", data = "<request>")]
pub async fn create(
    user: LoggedInUser,
    request: Json<Request>,
    db: &State<Database>,
) -> Result<Json<Household>, DatabaseError> {
    let household = domain::household::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(request.name.clone()),
    }
    .insert(db.conn())
    .await
    .map_err(DatabaseError::from)?;
    let members = domain::household_member::ActiveModel {
        user_id: Set(user.id),
        household_id: Set(household.id),
    }
    .insert(db.conn())
    .await
    .map_err(DatabaseError::from)?;
    Ok(Household {
        id: household.id,
        name: household.name,
        members: vec![members.user_id],
    }.into())
}
