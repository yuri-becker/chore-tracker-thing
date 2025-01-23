use crate::domain;
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::response::Household;
use crate::http::api::FromModel;
use crate::infrastructure::database::Database;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, NotSet};
use uuid::Uuid;

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
) -> Result<Json<Household>, ApiError> {
    let household = domain::household::ActiveModel {
        id: Set(Uuid::now_v7()),
        name: Set(request.name.clone()),
    }
    .insert(db.conn())
    .await
    .map_err(ApiError::from)?;
    domain::household_member::ActiveModel {
        user_id: Set(user.id),
        household_id: Set(household.id),
        joined_via_invite: NotSet,
    }
    .insert(db.conn())
    .await
    .map_err(ApiError::from)?;
    Household::from_model(db, household)
        .await
        .map(Json::from)
        .map_err(ApiError::from)
}

#[cfg(test)]
mod test {
    use crate::test_environment::TestEnvironment;
    use rocket::http::Status;
    use rocket::serde::json::json;
    use rocket::{async_test, routes, uri};

    #[async_test]
    async fn test_create_household() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![super::create]);
        let env = env.launch().await;

        let header = env.header_user_a();
        let response = env
            .post(uri!(super::create))
            .header(header)
            .json(&json!( {
              "name": "Whacky House"
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }
}
