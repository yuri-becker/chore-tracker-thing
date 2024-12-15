pub use sea_orm_migration::prelude::*;
use crate::infrastructure::database::Database;

mod m000001_create_oidc_user;
mod m000002_create_households;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000001_create_oidc_user::Migration),
            Box::new(m000002_create_households::Migration)
        ]
    }
}
pub async fn migrate(db: &Database) {
    Migrator::up(db.conn(), None).await.expect("Migration failed")
}
