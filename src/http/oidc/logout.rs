use crate::infrastructure::oidc_client::OidcClient;
use log::info;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::{get, State};
use crate::infrastructure::host::Host;

#[get("/logout")]
pub fn logout(
    cookie_jar: &CookieJar<'_>,
    oidc_client: &State<OidcClient>,
    host: Host,
) -> Redirect {
    cookie_jar.remove_private("user");
    if let Some(id_token) = cookie_jar.get("oidc_token") {
        let id_token = id_token.value();
        cookie_jar.remove_private("oidc_token");
        if let Some(session_endpoint) = &oidc_client.config().end_session_endpoint {
            let mut endpoint = session_endpoint.clone();
            endpoint
                .query_pairs_mut()
                .append_pair("id_token_hint", id_token)
                .append_pair(
                    "post_logout_redirect_uri",
                    (host.to_string() + "/").as_str(),
                );
            info!("Redirecting to OIDC logout endpoint: {}", endpoint);
            return Redirect::found::<String>(endpoint.to_string());
        };

    };
    Redirect::to("/")
}
