use std::ops::Deref;
use log::info;
use openid::{Client, Discovered, DiscoveredClient, StandardClaims};
use url::Url;
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
            config.oidc_client_id.clone(),
            config.oidc_client_password.clone(),
            config.host.clone() + "/oidc/callback",
            Url::parse(config.oidc_endpoint.as_str())
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
