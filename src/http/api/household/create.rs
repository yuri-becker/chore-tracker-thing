use crate::domain;
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::response::Household;
use crate::http::api::FromModel;
use crate::infrastructure::database::Database;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

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
    let household = domain::household::create(db, request.name.clone(), user.id)
        .await
        .map_err(ApiError::from)?;
    Household::from_model(db, household)
        .await
        .map(Json::from)
        .map_err(ApiError::from)
}

#[cfg(test)]
mod test {
    use crate::domain::{household, household_member};
    use crate::http::api::household::response::Household;
    use crate::test_environment::TestEnvironment;
    use rocket::http::Status;
    use rocket::serde::json::json;
    use rocket::{async_test, routes, uri};
    use sea_orm::EntityTrait;

    #[async_test]
    async fn test_create_household() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![super::create])
            .launch()
            .await;

        let response = env
            .post(uri!(super::create))
            .header(env.header_user_a())
            .json(&json!( {
              "name": "Whacky House"
            }))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let response: Household = response.into_json().await.unwrap();
        let entity = household::Entity::find_by_id(response.id)
            .one(env.database().conn())
            .await
            .unwrap()
            .expect("Household was not written into database");
        assert_eq!(entity.name, "Whacky House");

        let household_member = household_member::Entity::find_by_id((env.user_a, response.id))
            .one(env.database().conn())
            .await
            .unwrap()
            .expect("Member was not created");
        assert_eq!(household_member.joined_via_invite, None)
    }
}
