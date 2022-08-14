use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS `users` (
            `id` bigint NOT NULL AUTO_INCREMENT PRIMARY KEY,
            `account` varchar(30) NOT NULL DEFAULT '' COMMENT '帳號',
            `password` varchar(255) NOT NULL DEFAULT '' COMMENT '密碼',
            `token` varchar(255) NOT NULL DEFAULT '' COMMENT 'token',
            `name` varchar(30) NOT NULL DEFAULT '' COMMENT '暱稱',
            `role` tinyint(1) NOT NULL DEFAULT 1 COMMENT '權限 1 => user, 99 => admin',
            `api_key` varchar(100) NOT NULL DEFAULT '' COMMENT 'bybit api key',
            `secret_key` varchar(100) NOT NULL DEFAULT '' COMMENT 'bybit secret key',
            `state` tinyint(1) NOT NULL DEFAULT 1 COMMENT '狀態 0=>停用 1=>啟用',
            `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
            `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            `deleted_at` datetime DEFAULT NULL,
            INDEX (account)
        )"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE `users`";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
