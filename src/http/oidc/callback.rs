use crate::http::oidc::callback_query::CallbackQuery;
use crate::http::oidc::oidc_error::OidcError;
use crate::http::oidc::oidc_error::OidcError::{OidcEndpointUnreachable, Unauthorized};
use crate::infrastructure::oidc_client::OidcClient;
use crate::infrastructure::user::LoggedInUser;
use log::warn;
use openid::{CompactJson, StandardClaimsSubject, Token};
use openid::error::StandardClaimsSubjectMissing;
use rocket::http::{CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::serde::json::serde_json::to_string;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, State};
use rocket::http::private::cookie::CookieBuilder;

#[get("/callback")]
pub async fn callback<'r>(
    callback_query: CallbackQuery,
    oidc_client: &State<OidcClient>,
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
            let logged_in_user = LoggedInUser {
                name: user_info.preferred_username,
                groups: user_info.groups,
            };
            cookie_jar.add_private(
                CookieBuilder::new(
                    "user",
                    to_string(&logged_in_user).expect("Should always be parsable"),
                )
                    .same_site(SameSite::Lax)
                    .build(),
            );
            cookie_jar.add_private(
                CookieBuilder::new(
                    "token",
                    bearer.id_token.expect("Bearer should have id token"),
                )
                    .same_site(SameSite::Lax)
                    .build(),
            );
            cookie_jar.add(
                CookieBuilder::new("logged-in", "")
                    .same_site(SameSite::Lax)
                    .http_only(false)
                    .build()
            );
            Ok(Redirect::to("/"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct UserInfo {
    auth_time: u64,
    client_id: String,
    groups: Vec<String>,
    iss: String,
    preferred_username: String,
    sub: String,
}

impl CompactJson for UserInfo {}

impl StandardClaimsSubject for UserInfo {
    fn sub(&self) -> Result<&str, StandardClaimsSubjectMissing> {
        Ok(&self.sub)
    }
}