use rocket::{routes, Route};

mod callback;
mod callback_query;
mod login;
mod logout;
mod oidc_error;
#[cfg(test)]
mod oidc_test_environment;

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

    async fn get_dex_uri(env: &OidcTestEnvironment) -> String {
        let login_response = env.api().get("/oidc/login").dispatch().await;
        login_response
            .headers()
            .get_one("Location")
            .expect("Location should be set.")
            .to_string()
    }

    #[async_test]
    async fn test_login_new_user() {
        let env = OidcTestEnvironment::launch().await;
        let dex_uri = get_dex_uri(&env).await;
        let browser = env.dex_browser(&dex_uri).await;
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
    async fn test_login_existing_user_and_updates_display_name() {
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

        let dex_uri = get_dex_uri(&env).await;
        let browser = env.dex_browser(&dex_uri).await;
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
    async fn test_logout() {
        let env = OidcTestEnvironment::launch().await;
        let dex_uri = get_dex_uri(&env).await;
        let browser = env.dex_browser(&dex_uri).await;
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

    #[async_test]
    async fn missing_error_description_callback_throws_400() {
        let env = OidcTestEnvironment::launch().await;
        let response = env.api().get("/oidc/callback?error=ewwow").dispatch().await;
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[async_test]
    async fn missing_error_iss_throws_400() {
        let env = OidcTestEnvironment::launch().await;
        let response = env
            .api()
            .get("/oidc/callback?error=ewwow&error_description=something")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[async_test]
    async fn access_denied_callback_redirects_to_root() {
        let env = OidcTestEnvironment::launch().await;
        let response = env.api().get("/oidc/callback?error=access_denied&error_description=something&iss=http://dex.local").dispatch().await;
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(response.headers().get_one("Location"), Some("/"))
    }

    #[async_test]
    async fn unknown_error_callback_throws_500() {
        let env = OidcTestEnvironment::launch().await;
        let response = env.api().get("/oidc/callback?error=unknown_error&error_description=something&iss=http://dex.local").dispatch().await;
        assert_eq!(response.status(), Status::InternalServerError);
    }

    #[async_test]
    async fn no_code_throws_400() {
        let env = OidcTestEnvironment::launch().await;
        let response = env
            .api()
            .get("/oidc/callback?something=nothing")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[async_test]
    async fn invalid_token_throws_401() {
        let env = OidcTestEnvironment::launch().await;
        let response = env
            .api()
            .get("/oidc/callback?code=somethingsomething")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Unauthorized);
    }
}
