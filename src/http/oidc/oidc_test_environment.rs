use crate::http::api::guards::logged_in_user::OidcLoggedInUserResolver;
use crate::http::oidc::routes;
use crate::infrastructure::database::Database;
use crate::infrastructure::host::test::StaticHostAccessor;
use crate::infrastructure::oidc_client::OidcClient;
use crate::{init_dotenv, migration};
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use fantoccini::wd::Capabilities;
use fantoccini::{ClientBuilder, Locator};
use rocket::config::SecretKey;
use rocket::http::private::TcpListener;
use rocket::local::asynchronous::Client;
use rocket::serde::json::json;
use rocket::{Build, Config, Rocket};
use std::net::IpAddr;
use std::ops::Deref;
use std::process::Stdio;
use std::str::FromStr;
use std::time::Duration;
use testcontainers_modules::dex::Dex;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::ContainerAsync;
use testcontainers_modules::{dex, postgres};
use tokio::io::AsyncReadExt;
use tokio::process::{Child, Command};
use tokio::time::{sleep, timeout};

const CLIENT_ID: &str = "ctt-test";
const CLIENT_NAME: &str = "Chore Tracker Thing (test)";
const CLIENT_SECRET: &str = "secret!";
const BIND_ADDR: &str = "127.0.0.1";

pub struct OidcTestEnvironment {
    client: Client,
    chromedriver: (Child, u16),
    _postgres: ContainerAsync<postgres::Postgres>,
    _dex: ContainerAsync<dex::Dex>,
}

impl OidcTestEnvironment {
    pub async fn launch() -> Self {
        init_dotenv();
        let (postgres, database) = Self::build_database().await;
        let rocket_port = Self::find_free_port().await;
        let origin = format!("http://{BIND_ADDR}:{rocket_port}");
        let dex = Self::build_dex_container(&origin).await;
        let oidc_client = Self::build_oidc_client(&origin, &dex).await;
        let rocket = Self::build_rocket(database, rocket_port, origin.clone(), oidc_client);
        let chromedriver = Self::build_chromedriver().await;
        Self {
            client: Client::tracked(rocket).await.unwrap(),
            chromedriver,
            _postgres: postgres,
            _dex: dex,
        }
    }

    async fn build_database() -> (ContainerAsync<postgres::Postgres>, Database) {
        let postgres = postgres::Postgres::default().start().await.unwrap();
        let database = Database::connect_to_testcontainer(&postgres).await;
        migration::migrate(&database).await;
        (postgres, database)
    }

    async fn build_dex_container(origin: &str) -> ContainerAsync<Dex> {
        dex::Dex::default()
            .with_simple_user()
            .with_client(dex::PrivateClient {
                id: CLIENT_ID.to_string(),
                name: CLIENT_NAME.to_string(),
                redirect_uris: vec![format!("{origin}/oidc/callback")],
                secret: CLIENT_SECRET.to_string(),
            })
            .start()
            .await
            .unwrap()
    }

    async fn build_oidc_client(origin: &str, dex: &ContainerAsync<Dex>) -> OidcClient {
        OidcClient::new(
            CLIENT_ID.to_string(),
            CLIENT_SECRET.to_string(),
            origin.to_string(),
            format!(
                "http://{}:{}",
                dex.get_host().await.unwrap(),
                dex.get_host_port_ipv4(5556).await.unwrap()
            ),
        )
        .await
    }

    pub fn api(&self) -> &Client {
        &self.client
    }

    pub fn db(&self) -> &Database {
        self.client.rocket().state::<Database>().unwrap()
    }

    pub async fn dex_browser(&self, location: &str) -> DexBrowser {
        DexBrowser::new(self.chromedriver.1, location).await
    }

    fn build_rocket(
        database: Database,
        rocket_port: u16,
        host: String,
        oidc_client: OidcClient,
    ) -> Rocket<Build> {
        Rocket::build()
            .mount("/oidc", routes())
            .manage(oidc_client)
            .manage(database)
            .manage(OidcLoggedInUserResolver::new_state())
            .manage(StaticHostAccessor::new_state(host))
            .configure(Config {
                port: rocket_port,
                secret_key: SecretKey::from(
                    "extremely secret secret in oidc functionality integration test with dex"
                        .as_bytes(),
                ),
                address: IpAddr::from_str(BIND_ADDR).unwrap(),
                ..Config::default()
            })
    }

    async fn build_chromedriver() -> (Child, u16) {
        let port = Self::find_free_port().await;
        let chromedriver = std::env::var("CHROMEDRIVER").unwrap_or(String::from("chromedriver"));
        let mut child = Command::new(chromedriver)
            .process_group(0)
            .arg(format!("--port={port}"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        match timeout(Duration::from_millis(500), child.wait()).await {
            Ok(status) => {
                if let Some(status_code) = status.expect("child.wait() errored").code() {
                    panic!("chromedriver exited with code {}", status_code);
                }
                let mut stderr = String::new();
                child
                    .stderr
                    .expect("chromedriver did fail but not output to stderr")
                    .read_to_string(&mut stderr)
                    .await
                    .expect("Failed to read stderr");
                panic!("chromedriver failed to start: {}", stderr)
            }
            Err(_) => {
                // This branch means the timeout occurred, so the chromedriver did not exit –  this
                // is the positive case.
                (child, port)
            }
        }
    }

    async fn find_free_port() -> u16 {
        TcpListener::bind((BIND_ADDR, 0))
            .await
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }
}

pub struct DexBrowser {
    client: fantoccini::Client,
}

impl Deref for DexBrowser {
    type Target = fantoccini::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DexBrowser {
    async fn new(chromedriver_port: u16, goto: &str) -> Self {
        let mut caps = Capabilities::new();
        let headless = std::env::var("CHROMEDRIVER_HEADLESS")
            .unwrap_or("true".to_string())
            .parse::<bool>()
            .expect("CHROMEDRIVER_HEADLESS must be a boolean.");
        let no_sandbox = std::env::var("CHROMEDRIVER_NO_SANDBOX")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .expect("CHROMEDRIVER_NO_SANDBOX must be a boolean.");
        caps.insert(
            "goog:chromeOptions".to_string(),
            json!({"args": vec![
                if no_sandbox {"--no-sandbox"} else {""},
                if headless {"--headless"} else {""}
            ].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>() }),
        );

        let client = ClientBuilder::native()
            .capabilities(caps)
            .connect(&format!("http://localhost:{}", chromedriver_port))
            .await
            .expect("Could not connect to Chromedriver");

        client.goto(goto).await.unwrap();
        Self { client }
    }

    pub async fn wait_for_loaded(&self) -> Result<Element, CmdError> {
        self.client
            .wait()
            .for_element(Locator::Css(".dex-container"))
            .await
    }

    pub async fn login(&self) -> Result<(), CmdError> {
        self.client
            .find(Locator::Id("login"))
            .await?
            .send_keys("user@example.org")
            .await?;
        self.client
            .find(Locator::Id("password"))
            .await?
            .send_keys("user")
            .await?;
        self.client
            .find(Locator::Id("submit-login"))
            .await?
            .click()
            .await
    }

    pub async fn grant_access(&self) -> Result<(), CmdError> {
        let grant_access_button = Locator::Css("button.dex-btn.theme-btn--success[type=submit]");
        self.client.wait().for_element(grant_access_button).await?;
        self.client.find(grant_access_button).await?.click().await?;
        sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    pub async fn parse_callback_url(&self) -> Result<String, CmdError> {
        let callback_url = self.client.current_url().await?;
        let callback_path = callback_url.path();
        let callback_query = callback_url.query().expect("Should have query.");
        Ok(format!("{callback_path}?{callback_query}"))
    }
}
