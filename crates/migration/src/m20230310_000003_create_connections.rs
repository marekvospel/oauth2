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
                    .table(Connections::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Connections::Id)
                            .big_integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Connections::User).big_integer().not_null())
                    .col(
                        ColumnDef::new(Connections::ConnectionId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Connections::ConnectionType)
                            .string()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .col(Connections::ConnectionType)
                            .col(Connections::ConnectionId)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Connections::Table, Connections::User)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Connections::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Connections {
    Table,
    Id,
    User,
    ConnectionId,
    ConnectionType,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
