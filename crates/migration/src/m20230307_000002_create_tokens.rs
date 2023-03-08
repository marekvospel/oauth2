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
                    .table(Tokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tokens::Token)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tokens::Refresh).string().unique_key())
                    .col(
                        ColumnDef::new(Tokens::TokenType)
                            .string()
                            .not_null()
                            .default("bearer"),
                    )
                    .col(
                        ColumnDef::new(Tokens::Expire)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Tokens::Owner).big_integer().not_null())
                    .col(ColumnDef::new(Tokens::ClientId).big_integer())
                    .col(ColumnDef::new(Tokens::Scope).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tokens::Table, Tokens::Owner)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Tokens::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Tokens {
    Table,
    Token,
    Refresh,
    TokenType,
    Owner,
    Expire,
    ClientId,
    Scope,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
