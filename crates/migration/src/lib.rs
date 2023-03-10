pub use sea_orm_migration::prelude::*;

mod m20230306_000001_create_users;
mod m20230307_000002_create_tokens;
mod m20230310_000003_create_connections;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230306_000001_create_users::Migration),
            Box::new(m20230307_000002_create_tokens::Migration),
            Box::new(m20230310_000003_create_connections::Migration),
        ]
    }
}
