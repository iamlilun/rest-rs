use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS `strategies` (
            `name` varchar(30) PRIMARY KEY NOT NULL COMMENT '策略名稱',
            `symbol_name` varchar(30) NOT NULL COMMENT '綁定合約',
            `state` tinyint(1) NOT NULL DEFAULT 0 COMMENT '策略狀態 0 => 停用, 1=>啟用',
            `side` tinyint(1) NOT NULL COMMENT '下單方向 1 => 多單, 2 => 空單"',
            `Remark` varchar(255) NOT NULL DEFAULT '' COMMENT '備註',
            `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
            `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            Index(symbol_name)
        )"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE `strategies`";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
