pub use sea_orm_migration::prelude::*;
use crate::infrastructure::database::Database;

mod m20220101_000001_create_oidc_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_oidc_user::Migration)]
    }
}
pub async fn migrate(db: &Database) {
    Migrator::up(db.conn(), None).await.expect("Migration failed")
}
