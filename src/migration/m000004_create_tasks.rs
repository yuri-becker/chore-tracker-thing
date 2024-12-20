use sea_orm::sea_query::extension::postgres::Type;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{pk_uuid, uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(
            Type::create()
                .as_enum(Alias::new("recurrence_unit"))
                .values([Alias::new("Days"), Alias::new("Weeks"), Alias::new("Month")])
                .to_owned()
        ).await?;
        manager.create_table(
            Table::create()
                .table(Task::Table)
                .col(pk_uuid(Task::Id))
                .col(uuid(Task::HouseholdId))
                .col(ColumnDef::new(Task::Title).string().not_null())
                .col(ColumnDef::new(Task::RecurrenceUnit).custom(RecurrenceUnit::Type).not_null())
                .col(ColumnDef::new(Task::RecurrenceInterval).integer().not_null().check(Expr::col(Task::RecurrenceInterval).gte(1)))
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_TASKS_HOUSEHOLD")
                        .from(Task::Table, Task::HouseholdId)
                        .to(Household::Table, Household::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Task::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(RecurrenceUnit::Type).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Task {
    #[sea_orm(iden = "tasks")]
    Table,
    Id,
    HouseholdId,
    Title,
    RecurrenceUnit,
    RecurrenceInterval,
}

#[derive(DeriveIden)]
enum Household {
    #[sea_orm(iden = "households")]
    Table,
    Id,
}

#[derive(DeriveIden)]
enum RecurrenceUnit {
    #[sea_orm(iden = "recurrence_unit")]
    Type
}