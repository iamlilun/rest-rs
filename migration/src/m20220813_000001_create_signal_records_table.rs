use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS `signal_records` (
            `id` bigint NOT NULL AUTO_INCREMENT PRIMARY KEY,
            `strategy_name` varchar(30) NOT NULL COMMENT '策略名稱',
            `state` tinyint(1) NOT NULL DEFAULT 0 COMMENT '訂單狀態 1=>持倉中, 2=>已平倉',
            `side` tinyint(1) NOT NULL COMMENT '下單方向 1 => 多單, 2 => 空單"',
            `open_price` double NOT NULL DEFAULT 0 COMMENT '開倉價位',
            `close_price` double NOT NULL DEFAULT 0 COMMENT '平倉價位',
            `profit_and_loss` double NOT NULL DEFAULT 0 COMMENT '盈虧',
            `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
            `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE `signal_records`";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
