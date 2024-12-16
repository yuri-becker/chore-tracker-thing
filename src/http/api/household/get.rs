use crate::domain::{household, household_member};
use crate::http::api::household::response::Household;
use crate::http::error::database_error::DatabaseError;
use crate::infrastructure::database::Database;
use crate::infrastructure::user::LoggedInUser;
use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::entity::prelude::*;
use sea_orm::{ColumnTrait, QueryFilter};
use std::iter::Iterator;
use rocket::futures::future::try_join_all;

#[get("/")]
pub async fn get(
    db: &State<Database>,
    user: LoggedInUser,
) -> Result<Json<Vec<Household>>, DatabaseError> {
    let memberships = household_member::Entity::find()
        .find_also_related(household::Entity)
        .filter(household_member::Column::UserId.eq(user.id))
        .all(db.conn())
        .await
        .map_err(DatabaseError::from)?;

    let households = memberships.iter().map(|it| {
        let household = it.clone().1.expect("A HouseholdMembership without a Household should be prevented by foreign key constraints");
        Household::from_model(db, household)
    }).collect::<Vec<_>>();

    let households: Vec<Household> = try_join_all(households).await?;
    Ok(Json::from(households))
}