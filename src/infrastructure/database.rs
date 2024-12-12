use crate::infrastructure::config::Config;
use log::debug;
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
        let it = sea_orm::Database::connect(opts).await
            .expect("Could not connect to database");
        it.ping().await.expect("Database connection is invalid.");
        debug!("Successfully connected to the database");
        Database { connection: it }
    }
}
