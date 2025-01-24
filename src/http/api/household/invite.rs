use crate::domain::invite;
use crate::domain::invite_secret::InviteSecret;
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::UuidParam;
use crate::infrastructure::database::Database;
use chrono::{DateTime, TimeDelta, Utc};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{error, get};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
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

#[cfg(test)]
mod test {
    use crate::domain::{household, household_member};
    use crate::http::api::household::invite::Response;
    use crate::test_environment::TestEnvironment;
    use rocket::http::Status;
    use rocket::{async_test, routes};
    use sea_orm::ActiveValue::Set;
    use sea_orm::{ActiveModelTrait, NotSet};
    use uuid::Uuid;

    #[async_test]
    async fn test_generate_invite() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![super::generate_invite])
            .launch()
            .await;

        let household = household::ActiveModel {
            id: Set(Uuid::now_v7()),
            name: Set("My Household".to_string()),
        }
        .insert(env.database().conn())
        .await
        .unwrap();

        household_member::ActiveModel {
            household_id: Set(household.id),
            user_id: Set(env.user_a),
            joined_via_invite: NotSet,
        }
        .insert(env.database().conn())
        .await
        .unwrap();

        let invite_1 = env
            .get(format!("/{}/invite", household.id))
            .header(env.header_user_a())
            .dispatch()
            .await;

        assert_eq!(invite_1.status(), Status::Ok);
        let invite_1: Response = invite_1.into_json().await.unwrap();

        let invite_2 = env
            .get(format!("/{}/invite", household.id))
            .header(env.header_user_a())
            .dispatch()
            .await;

        assert_eq!(invite_2.status(), Status::Ok);
        let invite_2: Response = invite_2.into_json().await.unwrap();

        assert_ne!(invite_1.invite_code, invite_2.invite_code);
        assert_ne!(invite_1.secret, invite_2.secret);
    }
}
