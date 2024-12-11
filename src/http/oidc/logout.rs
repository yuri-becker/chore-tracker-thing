use log::info;
use rocket::{get, State};
use rocket::http::CookieJar;
use rocket::response::Redirect;
use crate::infrastructure::config::Config;
use crate::infrastructure::oidc_client::OidcClient;

#[get("/logout")]
pub fn logout(
    cookie_jar: &CookieJar<'_>,
    oidc_client: &State<OidcClient>,
    config: &State<Config>,
) -> Redirect {
    cookie_jar.remove_private("user");
    cookie_jar.remove("logged-in");
    if let Some(id_token) = cookie_jar.get("token") {
        let id_token = id_token.value();
        cookie_jar.remove_private("token");
        if let Some(session_endpoint) = &oidc_client.config().end_session_endpoint {
            let mut endpoint = session_endpoint.clone();
            endpoint.query_pairs_mut()
                .append_pair("id_token_hint", id_token)
                .append_pair("post_logout_redirect_uri", (config.host.clone() + "/").as_str());
            info!("Redirecting to OIDC logout endpoint: {}", endpoint);
            return Redirect::found::<String>(endpoint.to_string())
        };
    };
    Redirect::to("/")
}
