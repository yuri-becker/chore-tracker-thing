use crate::infrastructure::database::Database;
pub use sea_orm_migration::prelude::*;

mod m000001_create_oidc_user;
mod m000002_create_households;
mod m000003_add_column_display_name;
mod m000004_create_tasks;
mod m000005_create_todos;
mod m000006_create_invites;
mod m000007_add_column_joined_via_invite;

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
            Box::new(m000007_add_column_joined_via_invite::Migration),
        ]
    }
}
pub async fn migrate(db: &Database) {
    Migrator::up(db.conn(), None)
        .await
        .expect("Migration failed")
}

#[cfg(test)]
mod test {
    use crate::infrastructure::database::Database;
    use crate::migration::Migrator;
    use rocket::async_test;
    use sea_orm::TransactionTrait;
    use sea_orm_migration::MigratorTrait;
    use testcontainers_modules::postgres;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;

    #[async_test]
    async fn test_down_migration() {
        let postgres = postgres::Postgres::default().start().await.unwrap();
        let database = Database::connect_to_testcontainer(&postgres).await;

        database
            .conn()
            .transaction(|tx| {
                Box::pin(async move {
                    Migrator::down(tx, None).await?;
                    Migrator::up(tx, None).await
                })
            })
            .await
            .expect("Migrations failed.")
    }
}
