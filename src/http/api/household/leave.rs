use crate::domain::household_member;
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::UuidParam;
use crate::infrastructure::database::Database;
use rocket::post;
use sea_orm::EntityTrait;

#[post("/<household_id>/leave")]
pub async fn leave(
    user: LoggedInUser,
    household_id: UuidParam,
    db: &Database,
) -> Result<(), ApiError> {
    user.in_household(db, *household_id).await?;
    household_member::Entity::delete_by_id((user.id, *household_id))
        .exec(db.conn())
        .await?;
    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::test_environment::{TestEnvironment, TestUser};
    use rocket::http::Status;
    use rocket::{async_test, routes};
    use sea_orm::{ActiveModelTrait, ColumnTrait, NotSet, QueryFilter, Set};

    #[async_test]
    async fn test_leave_household() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![leave])
            .launch()
            .await;

        let household = env.create_household(None, TestUser::A).await;
        household_member::ActiveModel {
            user_id: Set(env.user_b),
            household_id: Set(household.id),
            joined_via_invite: NotSet,
        }
        .insert(env.database().conn())
        .await
        .unwrap();

        let response = env
            .post(format!("/{}/leave", household.id))
            .header(env.header_user_a())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);

        let members = household_member::Entity::find()
            .filter(household_member::Column::HouseholdId.eq(household.id))
            .all(env.database().conn())
            .await
            .unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members.first().unwrap().user_id, env.user_b);
    }
}
