use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(HouseholdMembers::Table)
                    .add_column(
                        ColumnDef::new(HouseholdMembers::JoinedViaInvite)
                            .uuid()
                            .null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("FK_HOUSEHOLD_MEMBERS_INVITE")
                            .from_tbl(HouseholdMembers::Table)
                            .from_col(HouseholdMembers::JoinedViaInvite)
                            .to_tbl(Invite::Table)
                            .to_col(Invite::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(HouseholdMembers::Table)
                    .drop_column(HouseholdMembers::JoinedViaInvite)
                    .drop_foreign_key(Alias::new("FK_HOUSEHOLD_MEMBERS_INVITE"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum HouseholdMembers {
    #[sea_orm(iden = "household_members")]
    Table,
    JoinedViaInvite,
}

#[derive(DeriveIden)]
enum Invite {
    #[sea_orm(iden = "invites")]
    Table,
    Id,
}
