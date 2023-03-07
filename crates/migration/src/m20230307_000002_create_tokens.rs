use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(AccessTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AccessTokens::Token)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AccessTokens::Refresh).string().unique_key())
                    .col(ColumnDef::new(AccessTokens::Owner).big_integer().not_null())
                    .col(
                        ColumnDef::new(AccessTokens::Expire)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AccessTokens::ClientId).big_integer())
                    .col(ColumnDef::new(AccessTokens::Scope).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(AccessTokens::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum AccessTokens {
    Table,
    Token,
    Refresh,
    Owner,
    Expire,
    ClientId,
    Scope,
}
