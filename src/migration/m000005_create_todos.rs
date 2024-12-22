use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .col(ColumnDef::new(Todo::TaskId).uuid().not_null())
                    .col(ColumnDef::new(Todo::Iteration).integer().not_null())
                    .col(ColumnDef::new(Todo::DueDate).date().not_null())
                    .col(ColumnDef::new(Todo::CompletedOn).date_time().null())
                    .col(ColumnDef::new(Todo::CompletedBy).uuid().null())
                    .primary_key(
                        Index::create()
                            .name("PK_TASK_ITERATION")
                            .col(Todo::TaskId)
                            .col(Todo::Iteration)
                            .primary(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_TODO_TASK")
                            .from(Todo::Table, Todo::TaskId)
                            .to(Task::Table, Task::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_TODO_COMPLETED_BY")
                            .from(Todo::Table, Todo::CompletedBy)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todo {
    #[sea_orm(iden = "todos")]
    Table,
    TaskId,
    Iteration,
    DueDate,
    CompletedBy,
    CompletedOn,
}

#[derive(DeriveIden)]
enum Task {
    #[sea_orm(iden = "tasks")]
    Table,
    Id,
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
}
