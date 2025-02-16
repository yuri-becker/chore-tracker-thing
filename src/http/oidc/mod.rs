use rocket::{routes, Route};

mod callback;
mod callback_query;
mod login;
mod logout;
mod oidc_error;
#[cfg(test)]
pub mod oidc_test_environment;

pub fn routes() -> Vec<Route> {
    routes![login::login, callback::callback, logout::logout]
}

/// Testing for the entire OIDC functionality is done very high-level, since testing individual
/// endpoints is not possible without having gone through the flow up until that point.
#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::{oidc_user, user};

    use oidc_test_environment::*;
    use rocket::async_test;
    use rocket::http::Status;
    use sea_orm::ActiveValue::Set;
    use sea_orm::{ActiveModelTrait, EntityTrait, NotSet};
    use uuid::Uuid;

    #[async_test]
    async fn new_user_can_login() {
        let env = OidcTestEnvironment::launch().await;
        let browser = env.dex_browser().await;
        browser.wait_for_loaded().await.unwrap();
        browser.login().await.unwrap();
        browser.grant_access().await.unwrap();
        let callback_response = env
            .api()
            .get(browser.parse_callback_url().await.unwrap())
            .dispatch()
            .await;
        assert_eq!(callback_response.status(), Status::SeeOther);
        assert!(
            callback_response.cookies().get("oidc_token").is_some(),
            "Callback did not set oidc_token."
        );
        assert_eq!(
            callback_response.cookies().get("user").unwrap().value(),
            "{\"name\":\"User\"}"
        );
    }

    #[async_test]
    async fn existing_user_can_login_and_updates_display_name() {
        let env = OidcTestEnvironment::launch().await;
        let user = user::ActiveModel {
            id: Set(Uuid::now_v7()),
            display_name: NotSet,
        }
        .insert(env.db().conn())
        .await
        .unwrap();
        oidc_user::ActiveModel {
            user_id: Set(user.id),
            subject: Set(String::from("CgR1c2VyEgVsb2NhbA")),
        }
        .insert(env.db().conn())
        .await
        .unwrap();

        let browser = env.dex_browser().await;
        browser.wait_for_loaded().await.unwrap();
        browser.login().await.unwrap();
        browser.grant_access().await.unwrap();
        let callback_response = env
            .api()
            .get(browser.parse_callback_url().await.unwrap())
            .dispatch()
            .await;
        assert_eq!(callback_response.status(), Status::SeeOther);
        assert!(
            callback_response.cookies().get("oidc_token").is_some(),
            "Callback did not set oidc_token."
        );
        assert_eq!(
            callback_response.cookies().get("user").unwrap().value(),
            "{\"name\":\"User\"}"
        );
        let user = user::Entity::find_by_id(user.id)
            .one(env.db().conn())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user.display_name, Some(String::from("User")));
    }

    #[async_test]
    async fn can_login_and_logout() {
        let env = OidcTestEnvironment::launch().await;
        let browser = env.dex_browser().await;
        browser.wait_for_loaded().await.unwrap();
        browser.login().await.unwrap();
        browser.grant_access().await.unwrap();
        let callback_response = env
            .api()
            .get(browser.parse_callback_url().await.unwrap())
            .dispatch()
            .await;
        let cookies = callback_response.cookies().iter().cloned();
        let logout_response = env
            .api()
            .get("/oidc/logout")
            .cookies(cookies)
            .dispatch()
            .await;
        assert_eq!(logout_response.status(), Status::SeeOther);
        assert_eq!(logout_response.cookies().get("user").unwrap().value(), "");
        assert_eq!(
            logout_response.cookies().get("oidc_token").unwrap().value(),
            ""
        );
    }
}
