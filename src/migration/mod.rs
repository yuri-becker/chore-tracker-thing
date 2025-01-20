use crate::infrastructure::database::Database;
pub use sea_orm_migration::prelude::*;

mod m000001_create_oidc_user;
mod m000002_create_households;
mod m000003_add_column_display_name;
mod m000004_create_tasks;
mod m000005_create_todos;
mod m000006_create_invites;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000001_create_oidc_user::Migration),
            Box::new(m000002_create_households::Migration),
            Box::new(m000003_add_column_display_name::Migration),
            Box::new(m000004_create_tasks::Migration),
            Box::new(m000005_create_todos::Migration),
            Box::new(m000006_create_invites::Migration),
        ]
    }
}
pub async fn migrate(db: &Database) {
    Migrator::up(db.conn(), None)
        .await
        .expect("Migration failed")
}
