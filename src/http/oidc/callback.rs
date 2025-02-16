use crate::domain::oidc_user;
use crate::http::oidc::callback_query::CallbackQuery;
use crate::http::oidc::oidc_error::OidcError;
use crate::http::oidc::oidc_error::OidcError::{
    Misconfiguration, OidcEndpointUnreachable, Unauthorized,
};
use crate::infrastructure::database::Database;
use crate::infrastructure::oidc_client::{OidcClient, UserInfo};
use log::{debug, error, warn};
use openid::error::ClientError;
use openid::Token;
use rocket::http::private::cookie::CookieBuilder;
use rocket::http::{CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::serde::json::serde_json::json;
use rocket::{get, State};

#[get("/callback")]
pub async fn callback(
    callback_query: CallbackQuery,
    oidc_client: &State<OidcClient>,
    database: &State<Database>,
    cookie_jar: &CookieJar<'_>,
) -> Result<Redirect, OidcError> {
    match callback_query {
        CallbackQuery::Error(error) => {
            if error.error == "access_denied" {
                return Ok(Redirect::to("/"));
            }
            warn!(
                "Could not log in via OIDC, client is probably misconfigured:\n{}\n{}\nISS: {}",
                error.error, error.error_description, error.iss
            );
            Err(OidcError::Misconfiguration(()))
        }
        CallbackQuery::Success(success) => {
            let bearer = oidc_client
                .request_token(&success.code)
                .await
                .inspect_err(|err| warn!("Could not request OIDC token: {:?}", err))
                .map_err(|err| match err {
                    ClientError::OAuth2(_) => Unauthorized(()),
                    ClientError::Io(_) => OidcEndpointUnreachable(()),
                    ClientError::Url(_) => OidcEndpointUnreachable(()),
                    _ => Misconfiguration(()),
                })?;
            let mut token = Token::from(bearer.clone());
            let id_token = token.id_token.as_mut().ok_or(OidcEndpointUnreachable(()))?;
            oidc_client
                .decode_token(id_token)
                .map_err(|_| Unauthorized(()))?;
            oidc_client
                .validate_token(id_token, None, None)
                .map_err(|_| Unauthorized(()))?;
            let user_info = oidc_client
                .request_userinfo_custom::<UserInfo>(&token)
                .await
                .inspect_err(|err| error!("Could not request userinfo: {:?}", err))
                .map_err(|_| OidcEndpointUnreachable(()))?;
            debug!("userinfo received: {:?}", user_info);
            let display_name = user_info
                .name
                .filter(|it| !it.is_empty())
                .or(user_info.preferred_username)
                .unwrap_or(user_info.sub.clone());
            let user = oidc_user::get_or_register(database, user_info.sub, display_name)
                .await
                .map_err(|err| {
                    warn!("Could not register OIDC user: {}", err);
                    OidcError::DatabaseConnectionError(())
                })?;

            cookie_jar.add(
                CookieBuilder::new(
                    "user",
                    json!({"name": user.display_name.expect("Should be set after get_or_register")}).to_string()
                )
                    .same_site(SameSite::Lax)
                    .http_only(false)
                    .build(),
            );
            cookie_jar.add_private(
                CookieBuilder::new(
                    "oidc_token",
                    bearer.id_token.expect("Bearer should have id token"),
                )
                .same_site(SameSite::Lax)
                .build(),
            );
            Ok(Redirect::to("/"))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::http::oidc::oidc_test_environment::OidcTestEnvironment;
    use rocket::async_test;
    use rocket::http::Status;

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
}
