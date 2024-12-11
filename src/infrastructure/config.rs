use dotenv::var;
use rocket::config::SecretKey;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub struct Hidden(String);

#[derive(Debug)]
pub struct Config {
    pub oidc_endpoint: String,
    pub oidc_client_id: String,
    pub oidc_client_password: Hidden,
    pub host: String,
    pub secret: Hidden,
    port: String
}

impl Debug for Hidden {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("\"***\"").finish()
    }
}

impl Deref for Hidden {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Hidden {
    fn from(val: String) -> Self {
        Hidden(val)
    }
}

impl Config {
    pub fn from_dotenv() -> Self {
        Config {
            oidc_endpoint: var("CHORES_OIDC_ENDPOINT").expect("CHORES_OIDC_ENDPOINT must be set"),
            oidc_client_id: var("CHORES_OIDC_CLIENT_ID").expect("CHORES_OIDC_CLIENT_ID must be set"),
            oidc_client_password: var("CHORES_OIDC_CLIENT_PASSWORD")
                .expect("CHORES_OIDC_CLIENT_PASSWORD must be set")
                .into(),
            host: var("CHORES_HOST").expect("CHORES_HOST must be set"),
            secret: var("CHORES_SECRET").expect("CHORES_SECRET must be set").into(),
            port: var("CHORES_PORT").unwrap_or("8001".to_string())
        }
    }
}

impl From<&Config> for rocket::Config {
    fn from(val: &Config) -> Self {
        rocket::Config {
            secret_key: SecretKey::from(val.secret.as_bytes()),
            port: val.port.parse::<u16>().expect("CHORES_PORT must be a valid port number"),
            ..rocket::Config::default()
        }
    }
}
