use crate::domain::{household, household_member};
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::response::Household;
use crate::http::api::FromModel;
use crate::infrastructure::database::Database;
use rocket::futures::future::try_join_all;
use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::entity::prelude::*;
use sea_orm::{ColumnTrait, QueryFilter};
use std::iter::Iterator;

#[get("/")]
pub async fn get(
    db: &State<Database>,
    user: LoggedInUser,
) -> Result<Json<Vec<Household>>, ApiError> {
    let memberships = household_member::Entity::find()
        .find_also_related(household::Entity)
        .filter(household_member::Column::UserId.eq(user.id))
        .all(db.conn())
        .await
        .map_err(ApiError::from)?;

    let households = memberships.iter().map(|it| {
        let household = it.clone().1.expect("A HouseholdMembership without a Household should be prevented by foreign key constraints");
        Household::from_model(db, household)
    }).collect::<Vec<_>>();

    let households: Vec<Household> = try_join_all(households).await?;
    Ok(Json::from(households))
}

#[cfg(test)]
mod test {
    use crate::http::api::household::response::Household;
    use crate::test_environment::{TestEnvironment, TestUser};
    use rocket::http::Status;
    use rocket::{async_test, routes, uri};

    #[async_test]
    async fn test_only_returns_households_with_membership() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![super::get])
            .launch()
            .await;

        let household_with_membership = env.create_household(Some("My House"), TestUser::A).await;
        env.create_household(Some("Not my House"), TestUser::B)
            .await;

        let response = env
            .get(uri!(super::get))
            .header(env.header_user_a())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
        let response: Vec<Household> = response.into_json().await.unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].id, household_with_membership.id);
    }
}
