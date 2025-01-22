use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{pk_uuid, uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Invite::Table)
                    .col(pk_uuid(Invite::Id))
                    .col(uuid(Invite::HouseholdId))
                    .col(ColumnDef::new(Invite::SecretDigest).string().not_null())
                    .col(ColumnDef::new(Invite::CreatedBy).uuid().null())
                    .col(ColumnDef::new(Invite::ValidUntil).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_INVITES_HOUSEHOLDS")
                            .from(Invite::Table, Invite::HouseholdId)
                            .to(Household::Table, Household::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_INVITES_USERS")
                            .from(Invite::Table, Invite::CreatedBy)
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
            .drop_table(Table::drop().table(Invite::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Invite {
    #[sea_orm(iden = "invites")]
    Table,
    Id,
    SecretDigest,
    HouseholdId,
    CreatedBy,
    ValidUntil,
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Household {
    #[sea_orm(iden = "households")]
    Table,
    Id,
}
