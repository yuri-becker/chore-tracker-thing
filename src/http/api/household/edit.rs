use crate::domain::household;
use crate::http::api::api_error::ApiError;
use crate::http::api::guards::logged_in_user::LoggedInUser;
use crate::http::api::household::response::Household;
use crate::http::api::{FromModel, UuidParam};
use crate::infrastructure::database::Database;
use rocket::put;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Request {
    pub name: String,
}

#[put("/<household_id>", data = "<request>")]
pub async fn edit(
    db: &Database,
    user: LoggedInUser,
    request: Json<Request>,
    household_id: UuidParam,
) -> Result<Json<Household>, ApiError> {
    user.in_household(db, *household_id).await?;

    let household = household::Entity::find_by_id(*household_id)
        .one(db.conn())
        .await?
        .expect("Household should exist due to previous check");
    let mut household: household::ActiveModel = household.into();
    household.name = Set(request.name.clone());
    let household = household.update(db.conn()).await?;
    Ok(Json::from(Household::from_model(db, household).await?))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http::api::household::routes;
    use crate::test_environment::{TestEnvironment, TestUser};
    use rocket::async_test;
    use rocket::http::Status;
    use rocket::serde::json::serde_json::json;

    #[async_test]
    async fn test_edit_household() {
        let env = TestEnvironment::builder()
            .await
            .mount(routes![edit])
            .launch()
            .await;

        let household = env.create_household(Some("Old Name"), TestUser::A).await;

        let response = env
            .put(format!("/{}", household.id))
            .header(env.header_user_a())
            .json(&json!({
                "name": "New Name",
            }))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        let response: Household = response.into_json().await.unwrap();
        assert_eq!(response.name, "New Name");

        let household = household::Entity::find_by_id(household.id)
            .one(env.database().conn())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(household.name, "New Name");
    }
}
