use std::ops::Deref;
use log::info;
use openid::{Client, CompactJson, Discovered, DiscoveredClient, StandardClaims, StandardClaimsSubject};
use url::Url;
use rocket::serde::{Deserialize, Serialize};
use openid::error::StandardClaimsSubjectMissing;
use crate::infrastructure::config::Config;

pub struct OidcClient {
    client: Client<Discovered, StandardClaims>
}

impl Deref for OidcClient {
    type Target = Client<Discovered, StandardClaims>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl OidcClient {
    pub async fn new(config: &Config) ->  Self {
        let client = DiscoveredClient::discover(
            config.oidc.client_id.clone(),
            (*config.oidc.client_password).clone(),
            config.host.clone() + "/oidc/callback",
            Url::parse(config.oidc.endpoint.as_str())
                .expect("oidc_endpoint is not a valid URL"),
        ).await;
        if client.is_err() {
            panic!("Failed to discover OIDC provider: {:?}", client.unwrap_err());
        }
        let client = client.unwrap();
        info!("Successfully discovered OIDC provider");
        OidcClient { client }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    pub auth_time: u64,
    pub client_id: String,
    pub groups: Vec<String>,
    pub iss: String,
    pub preferred_username: String,
    pub sub: String,
}

impl CompactJson for UserInfo {}

impl StandardClaimsSubject for UserInfo {
    fn sub(&self) -> Result<&str, StandardClaimsSubjectMissing> {
        Ok(&self.sub)
    }
}