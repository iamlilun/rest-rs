pub use sea_orm_migration::prelude::*;

mod m20220812_000001_create_users_table;
mod m20220812_000002_create_orders_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220812_000001_create_users_table::Migration),
            Box::new(m20220812_000002_create_orders_table::Migration),
        ]
    }
}
