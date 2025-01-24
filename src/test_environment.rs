use crate::domain::user;
use crate::http::api::guards::logged_in_user::test::TestLoggedInUserResolver;
use crate::infrastructure::database::Database;
use crate::migration;
use ctor::dtor;
use rocket::config::SecretKey;
use rocket::http::Header;
use rocket::local::asynchronous::Client;
use rocket::tokio::sync::OnceCell;
use rocket::{Build, Rocket, Route};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait};
use std::ops::Deref;
use testcontainers_modules::postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::ContainerAsync;
use uuid::Uuid;

static POSTGRES: OnceCell<ContainerAsync<postgres::Postgres>> = OnceCell::const_new();
static SECRET: &str = "sip-thickness-canister-uptake-tinwork-starless-reporter-tiling-tasting";

#[dtor]
fn after_all() {
    // we need to manually stop and remove the container, since the OnceCell never gets dropped
    let id = POSTGRES.get().unwrap().id();
    std::process::Command::new("docker")
        .args(["stop", id])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    std::process::Command::new("docker")
        .args(["remove", id])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub struct TestEnvironmentBuilder {
    rocket: Rocket<Build>,
    user_a: Uuid,
    user_b: Uuid,
}

impl TestEnvironmentBuilder {
    pub fn mount<R>(mut self, routes: R) -> Self
    where
        R: Into<Vec<Route>>,
    {
        self.rocket = self.rocket.mount("/", routes);
        self
    }

    pub async fn launch(self) -> TestEnvironment {
        let client = Client::tracked(self.rocket).await.unwrap();

        TestEnvironment {
            client,
            user_a: self.user_a,
            user_b: self.user_b,
        }
    }
}

/// Common test infrastructure, starting a Rocket instance and necessary testcontainer(s).
/// It derefs to a rocket testing client and provides two test users.
///
/// ## Example
/// ```rust,norun
/// #[async_test]
/// async fn test_create_household() {
///     let env = TestEnvironment::builder()
///         .await
///         .mount(routes![super::get_household])
///         .launch()
///         .await;
///
///     let response = env
///         .get(uri!(super::get_household))
///         .header(env.header_user_a())
///         .dispatch()
///         .await;
///
///     assert_eq!(response.status(), Status::Ok);
/// ```
pub struct TestEnvironment {
    pub client: Client,
    pub user_a: Uuid,
    pub user_b: Uuid,
}

impl TestEnvironment {
    pub async fn builder() -> TestEnvironmentBuilder {
        let postgres = POSTGRES
            .get_or_init(|| async {
                let postgres = postgres::Postgres::default().start().await.unwrap();
                let database = Database::connect_to_testcontainer(&postgres).await;
                migration::migrate(&database).await;
                postgres
            })
            .await;

        let database = Database::connect_to_testcontainer(postgres).await;
        Self::reset_data(&database).await;
        let (user_a, user_b) = Self::create_test_users(&database).await;

        let rocket = Rocket::build()
            .configure(rocket::Config {
                port: 0, // automatically assign port
                secret_key: SecretKey::from(SECRET.as_bytes()),
                ..rocket::Config::default()
            })
            .manage(database)
            .manage(TestLoggedInUserResolver::new_state());

        TestEnvironmentBuilder {
            rocket,
            user_a,
            user_b,
        }
    }

    async fn reset_data(database: &Database) {
        database
            .conn()
            .execute_unprepared(
                "select 'drop table \"' || tablename || '\" cascade;' from pg_tables;",
            )
            .await
            .unwrap();
    }

    async fn create_test_users(database: &Database) -> (Uuid, Uuid) {
        let user_a = Uuid::now_v7();
        user::ActiveModel {
            id: Set(user_a),
            display_name: Set(Some("User A".to_string())),
        }
        .insert(database.conn())
        .await
        .unwrap();
        let user_b = Uuid::now_v7();
        user::ActiveModel {
            id: Set(user_b),
            display_name: Set(Some("User B".to_string())),
        }
        .insert(database.conn())
        .await
        .unwrap();
        (user_a, user_b)
    }

    pub fn header_user_a(&self) -> Header<'static> {
        Header::new("X-Test-User".to_string(), self.user_a.to_string())
    }

    pub fn header_user_b(&self) -> Header<'static> {
        Header::new("X-Test-User".to_string(), self.user_b.to_string())
    }

    pub fn database(&self) -> &Database {
        self.client.rocket().state::<Database>().unwrap()
    }
}

impl Deref for TestEnvironment {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
