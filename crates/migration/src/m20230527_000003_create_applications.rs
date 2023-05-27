use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Tokens::Table)
                    .rename_column(Tokens::ClientId, Tokens::ApplicationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Applications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Applications::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Applications::Secret)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Applications::Owner).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Applications::Table, Applications::Owner)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("tokens_owner_application_fk")
                    .from(Tokens::Table, Tokens::ApplicationId)
                    .to(Applications::Table, Applications::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("tokens_owner_application_fk")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Applications::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Tokens::Table)
                    .rename_column(Tokens::ApplicationId, Tokens::ClientId)
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Applications {
    Table,
    Id,
    Secret,
    Owner,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}

#[derive(Iden)]
enum Tokens {
    Table,
    ClientId,
    ApplicationId,
}
