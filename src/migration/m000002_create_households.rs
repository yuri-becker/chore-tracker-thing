use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Household::Table)
                    .col(pk_uuid(Household::Id))
                    .col(ColumnDef::new(Household::Name).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(HouseholdMembers::Table)
                    .col(uuid(HouseholdMembers::UserId))
                    .col(uuid(HouseholdMembers::HouseholdId))
                    .primary_key(
                        Index::create()
                            .name("PK_HOUSEHOLD_MEMBER")
                            .col(HouseholdMembers::UserId)
                            .col(HouseholdMembers::HouseholdId)
                            .primary()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_HOUSEHOLD_MEMBERS_USERS")
                            .from(HouseholdMembers::Table, HouseholdMembers::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_HOUSEHOLD_MEMBERS_HOUSEHOLD")
                            .from(Household::Table, HouseholdMembers::HouseholdId)
                            .to(Household::Table, Household::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(HouseholdMembers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Household::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Household {
    #[sea_orm(iden = "households")]
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum HouseholdMembers {
    #[sea_orm(iden = "household_members")]
    Table,
    UserId,
    HouseholdId,
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    Id,
}
