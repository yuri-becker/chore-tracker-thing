use crate::infrastructure::config::Config;
use crate::migration::async_trait::async_trait;
use log::debug;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use sea_orm::{ConnectOptions, DatabaseConnection};

pub struct Database {
    connection: DatabaseConnection,
}

impl Database {
    pub fn conn(&self) -> &DatabaseConnection {
        &self.connection
    }
}

impl Database {
    pub async fn connect(config: &Config) -> Database {
        let opts = ConnectOptions::new(config.postgres.build_connection_string());
        let it = sea_orm::Database::connect(opts)
            .await
            .expect("Could not connect to database");
        it.ping().await.expect("Database connection is invalid.");
        debug!("Successfully connected to the database");
        Database { connection: it }
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for &'r Database {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(
            request
                .rocket()
                .state::<Database>()
                .expect("Database should be in managed state"),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::database::Database;
    use sea_orm::{DatabaseBackend, MockDatabase};

    impl Database {
        pub fn mock() -> MockDatabase {
            MockDatabase::new(DatabaseBackend::Postgres)
        }
    }

    impl From<MockDatabase> for Database {
        fn from(val: MockDatabase) -> Self {
            Database {
                connection: val.into_connection()
            }
        }
    }
}
