use crate::infrastructure::config::Config;
use crate::migration::async_trait::async_trait;
use log::debug;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use sea_orm::{ConnectOptions, DatabaseConnection};

#[derive(Debug)]
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
        Self::connect_string(config.postgres.build_connection_string()).await
    }

    async fn connect_string(connection: String) -> Database {
        let opts = ConnectOptions::new(connection);
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
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers::ContainerAsync;

    impl Database {
        pub async fn connect_to_testcontainer(container: &ContainerAsync<Postgres>) -> Database {
            let connection_string = format!(
                "postgres://postgres:postgres@{}:{}/postgres",
                container.get_host().await.unwrap(),
                container.get_host_port_ipv4(5432).await.unwrap()
            );
            Database::connect_string(connection_string).await
        }
    }
}
