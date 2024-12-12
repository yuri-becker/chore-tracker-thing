use crate::http::oidc::callback_query::CallbackQuery;
use crate::http::oidc::oidc_error::OidcError;
use crate::http::oidc::oidc_error::OidcError::{OidcEndpointUnreachable, Unauthorized};
use crate::infrastructure::oidc_client::{OidcClient, UserInfo};
use log::warn;
use openid::Token;
use rocket::http::{CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::serde::json::serde_json::json;
use rocket::{get, State};
use rocket::http::private::cookie::CookieBuilder;
use crate::domain::oidc_user;
use crate::infrastructure::database::Database;

#[get("/callback")]
pub async fn callback<'r>(
    callback_query: CallbackQuery,
    oidc_client: &State<OidcClient>,
    database: &State<Database>,
    cookie_jar: &'r CookieJar<'_>,
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
                .map_err(|_| OidcEndpointUnreachable(()))?;
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
                .map_err(|_| OidcEndpointUnreachable(()))?;
            oidc_user::get_or_register(database, user_info.sub).await.map_err(|err| {
                warn!("Could not register OIDC user: {}", err);
                OidcError::DatabaseConnectionError(())
            })?;

            cookie_jar.add(
                CookieBuilder::new(
                    "user",
                    json!({"name": user_info.preferred_username}).to_string()
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

