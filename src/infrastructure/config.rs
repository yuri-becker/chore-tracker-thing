use dotenv::var;
use rocket::config::SecretKey;
use rocket::figment::Profile;
use std::fmt::{Debug, Display, Formatter};
use std::net::IpAddr;
use std::ops::Deref;
use std::str::FromStr;

pub struct ModeNotAvailableError();

impl Debug for ModeNotAvailableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CHORES_MODE must be either \"debug\" or \"prod\"")
    }
}
#[derive(Debug, Clone, Default)]
pub enum Mode {
    Debug,
    #[default]
    Prod,
}

impl FromStr for Mode {
    type Err = ModeNotAvailableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(Self::Debug),
            "prod" => Ok(Self::Prod),
            _ => Err(ModeNotAvailableError()),
        }
    }
}

impl From<Mode> for Profile {
    fn from(val: Mode) -> Self {
        match val {
            Mode::Debug => rocket::Config::DEBUG_PROFILE,
            Mode::Prod => rocket::Config::RELEASE_PROFILE,
        }
    }
}

#[derive(Clone)]
pub struct Hidden(String);

#[derive(Debug)]
pub struct Config {
    pub mode: Mode,
    pub oidc: Oidc,
    pub postgres: Postgres,
    pub host: String,
    secret: Hidden,
    port: u16,
}

#[derive(Debug)]
pub struct Oidc {
    pub endpoint: String,
    pub client_id: String,
    pub client_password: Hidden,
}

#[derive(Debug)]
pub struct Postgres {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Option<Hidden>,
    pub database: String,
}

impl Display for Hidden {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
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
            oidc: Oidc::from_dotenv(),
            host: var("CHORES_HOST").expect("CHORES_HOST must be set"),
            secret: var("CHORES_SECRET")
                .expect("CHORES_SECRET must be set")
                .into(),
            postgres: Postgres::from_dotenv(),
            port: var("CHORES_PORT")
                .map(|it| {
                    it.parse::<u16>()
                        .expect("CHORES_PORT must be a valid port number")
                })
                .unwrap_or(8001),
            mode: var("CHORES_MODE")
                .map(|it| Mode::from_str(&it).unwrap())
                .unwrap_or_default(),
        }
    }
}

impl Oidc {
    pub fn from_dotenv() -> Self {
        Oidc {
            endpoint: var("CHORES_OIDC_ENDPOINT").expect("CHORES_OIDC_ENDPOINT must be set"),
            client_id: var("CHORES_OIDC_CLIENT_ID").expect("CHORES_OIDC_CLIENT_ID must be set"),
            client_password: var("CHORES_OIDC_CLIENT_PASSWORD")
                .expect("CHORES_OIDC_CLIENT_PASSWORD must be set")
                .into(),
        }
    }
}

impl Postgres {
    pub fn from_dotenv() -> Self {
        Self {
            host: var("CHORES_POSTGRES_HOST").unwrap_or("127.0.0.1".to_string()),
            port: var("CHORES_POSTGRES_PORT")
                .map(|it| {
                    it.parse::<u16>()
                        .expect("CHORES_POSTGRES_PORT must be a valid port number")
                })
                .unwrap_or(3306),
            user: var("CHORES_POSTGRES_USER").expect("CHORES_POSTGRES_USER must be set"),
            password: var("CHORES_POSTGRES_PASSWORD").ok().map(Hidden::from),
            database: var("CHORES_POSTGRES_DATABASE")
                .expect("CHORES_POSTGRES_DATABASE must be set"),
        }
    }

    pub fn build_connection_string(&self) -> String {
        format!(
            "postgres://{}{}@{}:{}/{}",
            self.user,
            self.password
                .clone()
                .map(|it| format!(":{}", it))
                .unwrap_or_default(),
            self.host,
            self.port,
            self.database
        )
    }
}

impl From<&Config> for rocket::Config {
    fn from(val: &Config) -> Self {
        rocket::Config {
            secret_key: SecretKey::from(val.secret.as_bytes()),
            port: val.port,
            profile: val.mode.clone().into(),
            address: IpAddr::from_str("0.0.0.0").unwrap(),
            ..rocket::Config::default()
        }
    }
}
