use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS `orders` (
            `order_link_id` varchar(36) NOT NULL PRIMARY KEY COMMENT '自訂義的id',
            `order_id` varchar(100) NOT NULL COMMENT 'bybit order id',
            `side` tinyint(1) NOT NULL COMMENT '下單方向 1 => 多單, 2 => 空單"',
            `symbol` varchar(30) NOT NULL COMMENT '所屬合約',
            `price` double NOT NULL DEFAULT 0 COMMENT '價格',
            `qty` double NOT NULL DEFAULT 0 COMMENT '數量',
            `order_type` varchar(10) NOT NULL DEFAULT 'Limit' COMMENT 'Limit => 限價單, Market => 市價單',
            `reduce_only` boolean COMMENT '是否平倉單 true是 false否',
            `kline_time` integer COMMENT 'K線周期時間.以分鐘計',
            `profit_and_loss` double NOT NULL DEFAULT 0 COMMENT '盈虧',
            `rel_order_id` varchar(100) NOT NULL DEFAULT '' COMMENT '內部關聯的order_id',
            `rel_order_link_id` varchar(36) NOT NULL DEFAULT '' COMMENT '內部關聯的自訂義的id',
            `user_account` varchar(30) NOT NULL DEFAULT '' COMMENT '使用者帳號',
            `strategy_name` varchar(30) NOT NULL DEFAULT '' COMMENT '觸發下單的策略名稱',
            `action` tinyint(1) NOT NULL DEFAULT 1 COMMENT '訂單操作 1=>開倉 2=>平倉',
            `state` tinyint(1) NOT NULL DEFAULT 0 COMMENT '訂單狀態 0=>排隊中 1=>部分持倉 2=>全部持倉 3=>已平倉 4=>資料已確認 5=>超時.取消訂單',
            `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
            `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            INDEX (state),
            INDEX (user_account),
            INDEX (strategy_name)
        )"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE `orders`";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
