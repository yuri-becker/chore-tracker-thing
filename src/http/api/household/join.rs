use crate::domain;
use crate::domain::invite_secret::InviteSecret;
use crate::domain::{household_member, invite};
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::response::Household;
use crate::http::api::FromModel;
use crate::infrastructure::database::Database;
use domain::household;
use log::debug;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

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
        .filter(invite::Column::ValidUntil.gte(chrono::Utc::now()))
        .one(db.conn())
        .await?;
    let invite = invite.ok_or(ApiError::NotFound(()))?;

    let valid = InviteSecret {
        secret: request.secret.clone(),
        digest: invite.secret_digest.clone(),
    }
    .validate();
    if !valid {
        return Err(ApiError::NotFound(()));
    }

    let user_already_in_household = household_member::Entity::find()
        .filter(
            Condition::all()
                .add(household_member::Column::HouseholdId.eq(invite.household_id))
                .add(household_member::Column::UserId.eq(user.id)),
        )
        .count(db.conn())
        .await?
        > 0;
    if user_already_in_household {
        return Err(ApiError::Conflict(()));
    }

    household_member::ActiveModel {
        user_id: Set(user.id),
        household_id: Set(invite.household_id),
        joined_via_invite: Set(Some(invite.id)),
    }
    .insert(db.conn())
    .await?;

    let household = household::Entity::find_by_id(invite.household_id)
        .one(db.conn())
        .await?
        .expect("Household is ensured to exist by here.");

    let household = Household::from_model(db, household).await?;
    Ok(Json::from(household))
}

#[cfg(test)]
mod test {
    use crate::domain::household::Model;
    use crate::domain::household_member;
    use crate::http::api::household::invite;
    use crate::test_environment::{TestEnvironment, TestUser};
    use rocket::http::Status;
    use rocket::serde::json::serde_json::json;
    use rocket::{async_test, routes, uri};
    use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

    struct Helpers {}

    impl Helpers {
        async fn create_env() -> TestEnvironment {
            TestEnvironment::builder()
                .await
                .mount(routes![super::join, invite::generate_invite])
                .launch()
                .await
        }

        async fn generate_household_with_invite(
            env: &TestEnvironment,
        ) -> (Model, invite::Response) {
            let household = env.create_household(None, TestUser::A).await;
            let invite: invite::Response = env
                .get(format!("/{}/invite", household.id))
                .header(env.header_user_a())
                .dispatch()
                .await
                .into_json()
                .await
                .unwrap();
            (household, invite)
        }
    }

    #[async_test]
    async fn test_not_found_when_invite_code_is_not_a_uuid() {
        let env = Helpers::create_env().await;
        let response = env
            .post(uri!(super::join))
            .header(env.header_user_a())
            .json(&json!({
                "invite_code": "nouuid",
                "secret": "supersecret"
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::NotFound);
    }

    #[async_test]
    async fn test_not_found_when_invite_doesnt_exist() {
        let env = Helpers::create_env().await;
        let response = env
            .post(uri!(super::join))
            .header(env.header_user_a())
            .json(&json!({
                "invite_code": "01949805-aa2a-7229-9935-dcd599d220cb",
                "secret": "supersecret"
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::NotFound);
    }

    #[async_test]
    async fn test_not_found_when_secret_invalid() {
        let env = Helpers::create_env().await;
        let (_, invite) = Helpers::generate_household_with_invite(&env).await;

        let response = env
            .post(uri!(super::join))
            .header(env.header_user_a())
            .json(&json!({
                "invite_code": invite.invite_code,
                "secret": format!("{}_invalid", invite.secret)
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::NotFound);
    }

    #[async_test]
    async fn test_conflict_when_user_in_household() {
        let env = Helpers::create_env().await;
        let (_, invite) = Helpers::generate_household_with_invite(&env).await;

        let response = env
            .post(uri!(super::join))
            .header(env.header_user_a())
            .json(&json!({
                "invite_code": invite.invite_code,
                "secret": invite.secret,
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Conflict);
    }

    #[async_test]
    async fn test_join() {
        let env = Helpers::create_env().await;
        let (household, invite) = Helpers::generate_household_with_invite(&env).await;

        let response = env
            .post(uri!(super::join))
            .header(env.header_user_b())
            .json(&json!({
                "invite_code": invite.invite_code,
                "secret": invite.secret
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let membership = household_member::Entity::find()
            .filter(
                Condition::all()
                    .add(household_member::Column::HouseholdId.eq(household.id))
                    .add(household_member::Column::UserId.eq(env.user_b)),
            )
            .one(env.database().conn())
            .await
            .unwrap()
            .expect("Membership should have been created.");

        assert!(membership.joined_via_invite.is_some());
        assert_eq!(membership.joined_via_invite.unwrap(), invite.invite_code);
    }
}
