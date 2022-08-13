pub use sea_orm_migration::prelude::*;

mod m20220812_000001_create_users_table;
mod m20220812_000002_create_orders_table;
mod m20220813_000001_create_signal_records_table;
mod m20220813_000002_create_strategies_table;
mod m20220813_000003_create_subscribes_table;
mod m20220813_000004_create_symbols_table;
mod m20220813_000005_create_order_errors_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220812_000001_create_users_table::Migration),
            Box::new(m20220812_000002_create_orders_table::Migration),
            Box::new(m20220813_000001_create_signal_records_table::Migration),
            Box::new(m20220813_000002_create_strategies_table::Migration),
            Box::new(m20220813_000003_create_subscribes_table::Migration),
            Box::new(m20220813_000004_create_symbols_table::Migration),
            Box::new(m20220813_000005_create_order_errors_table::Migration),
        ]
    }
}
