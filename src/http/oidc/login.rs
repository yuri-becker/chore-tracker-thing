use log::info;
use openid::Options;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::{get, State};
use rocket::http::Status;
use rocket::response::Redirect;
use crate::infrastructure::oidc_client::OidcClient;

#[get("/login")]
pub fn login(oidc_client: &State<OidcClient>) -> Result<Redirect, Status> {
    let state = String::from_utf8(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .collect::<Vec<u8>>(),
    )
    .unwrap();
    let nonce = String::from_utf8(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .collect::<Vec<u8>>(),
    )
    .unwrap();

    let options = Options {
        scope: Some(String::from("openid profile groups")),
        state: Some(state),
        nonce: Some(nonce),
        ..Options::default()
    };
    let url = oidc_client.auth_url(&options);
    info!("Redirecting to OIDC endpoint: {}", url);
    Ok(Redirect::found::<String>(url.into()))
}