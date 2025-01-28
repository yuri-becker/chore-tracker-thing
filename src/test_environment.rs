use crate::domain::household::Model;
use crate::domain::{household, user};
use crate::http::api::guards::logged_in_user::test::TestLoggedInUserResolver;
use crate::infrastructure::database::Database;
use crate::migration;
use rocket::config::SecretKey;
use rocket::fairing::Fairing;
use rocket::http::Header;
use rocket::local::asynchronous::Client;
use rocket::{Build, Rocket, Route};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use std::ops::Deref;
use testcontainers_modules::postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::ContainerAsync;
use uuid::Uuid;

static SECRET: &str = "sip-thickness-canister-uptake-tinwork-starless-reporter-tiling-tasting";

pub struct TestEnvironmentBuilder {
    rocket: Rocket<Build>,
    user_a: Uuid,
    user_b: Uuid,
    postgres: ContainerAsync<postgres::Postgres>,
}

impl TestEnvironmentBuilder {
    pub fn mount<R>(mut self, routes: R) -> Self
    where
        R: Into<Vec<Route>>,
    {
        self.rocket = self.rocket.mount("/", routes);
        self
    }

    pub fn attach<F: Fairing>(mut self, fairing: F) -> Self {
        self.rocket = self.rocket.attach(fairing);
        self
    }

    pub async fn launch(self) -> TestEnvironment {
        let client = Client::tracked(self.rocket).await.unwrap();

        TestEnvironment {
            client,
            user_a: self.user_a,
            user_b: self.user_b,
            _postgres: self.postgres,
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
    _postgres: ContainerAsync<postgres::Postgres>,
}

impl TestEnvironment {
    pub async fn builder() -> TestEnvironmentBuilder {
        let postgres = postgres::Postgres::default().start().await.unwrap();
        let database = Database::connect_to_testcontainer(&postgres).await;
        migration::migrate(&database).await;
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
            postgres,
        }
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

    pub async fn create_household(&self, name: Option<&'static str>, user: TestUser) -> Model {
        household::create(
            self.database(),
            name.unwrap_or("My Household").to_string(),
            user.id(self),
        )
        .await
        .unwrap()
    }
}

impl Deref for TestEnvironment {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

pub enum TestUser {
    A,
    B,
}

impl TestUser {
    pub fn id(&self, env: &TestEnvironment) -> Uuid {
        match self {
            TestUser::A => env.user_a,
            TestUser::B => env.user_b,
        }
    }
}
