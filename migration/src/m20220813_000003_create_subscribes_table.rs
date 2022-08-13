use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS `subscribes` (
            `id` bigint NOT NULL AUTO_INCREMENT PRIMARY KEY,
            `user_account` varchar(30) NOT NULL COMMENT '訂閱用戶',
            `strategy_name` varchar(30) NOT NULL COMMENT '策略名稱',
            `amount` double NOT NULL COMMENT '單筆交易金額-USDT',
            `is_isolated` boolean COMMENT '全倉還逐倉 true => 逐倉 false 全倉',
            `leverage` smallint NOT NULL DEFAULT 0 COMMENT '槓桿數',
            `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
            `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE `subscribes`";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
