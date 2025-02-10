use crate::infrastructure::config::Config;
use log::info;
use openid::error::StandardClaimsSubjectMissing;
use openid::{
    Client, CompactJson, Discovered, DiscoveredClient, StandardClaims, StandardClaimsSubject,
};
use rocket::serde::{Deserialize, Serialize};
use std::ops::Deref;
use url::Url;

pub struct OidcClient {
    client: Client<Discovered, StandardClaims>,
}

impl Deref for OidcClient {
    type Target = Client<Discovered, StandardClaims>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl OidcClient {
    pub async fn from_config(config: &Config) -> Self {
        Self::new(
            config.oidc.client_id.clone(),
            (*config.oidc.client_password).clone(),
            config.host.clone(),
            config.oidc.endpoint.clone(),
        )
        .await
    }

    pub async fn new(
        client_id: String,
        client_password: String,
        origin: String,
        endpoint: String,
    ) -> OidcClient {
        let client = DiscoveredClient::discover(
            client_id,
            client_password,
            origin + "/oidc/callback",
            Url::parse(&endpoint).expect("oidc_endpoint is not a valid URL"),
        )
        .await.expect("Failed to discover OIDC provider");
        info!("Successfully discovered OIDC provider");
        OidcClient { client }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    #[serde(default)]
    pub groups: Vec<String>,
    pub iss: String,
    pub preferred_username: Option<String>,
    pub sub: String,
    pub name: Option<String>,
}

impl CompactJson for UserInfo {}

impl StandardClaimsSubject for UserInfo {
    fn sub(&self) -> Result<&str, StandardClaimsSubjectMissing> {
        Ok(&self.sub)
    }
}
