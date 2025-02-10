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
    use fantoccini::Locator;
    use oidc_test_environment::*;
    use rocket::async_test;
    use rocket::http::Status;
    use std::time::Duration;
    use tokio::time::sleep;

    #[async_test]
    async fn test_login_new_user() {
        let env = OidcTestEnvironment::launch().await;

        // Get Dex Login URL
        let login_response = env.api().get("/oidc/login").dispatch().await;
        assert_eq!(login_response.status(), Status::Found);
        let login_location = login_response.headers().get_one("Location").unwrap();

        let browser = env.browser().await;
        browser.goto(login_location).await.unwrap();
        browser
            .wait()
            .for_element(Locator::Css(".dex-container"))
            .await
            .unwrap();
        // Enter login details and submit
        browser
            .find(Locator::Id("login"))
            .await
            .unwrap()
            .send_keys("user@example.org")
            .await
            .unwrap();
        browser
            .find(Locator::Id("password"))
            .await
            .unwrap()
            .send_keys("user")
            .await
            .unwrap();
        browser
            .find(Locator::Id("submit-login"))
            .await
            .unwrap()
            .click()
            .await
            .unwrap();

        // Click "Grant Access"
        let grant_access_button = Locator::Css("button.dex-btn.theme-btn--success[type=submit]");
        browser
            .wait()
            .for_element(grant_access_button)
            .await
            .unwrap();
        browser
            .find(grant_access_button)
            .await
            .unwrap()
            .click()
            .await
            .unwrap();

        sleep(Duration::from_secs(1)).await;
        let callback_url = browser.current_url().await.unwrap();
        let callback_path = callback_url.path();
        let callback_query = callback_url.query().expect("Should have query.");
        let callback_response = env
            .api()
            .get(format!("{callback_path}?{callback_query}"))
            .dispatch()
            .await;
        assert_eq!(callback_response.status(), Status::SeeOther);
        assert!(
            callback_response.cookies().get("oidc_token").is_some(),
            "Callback did not set oidc_token."
        );
    }
}
