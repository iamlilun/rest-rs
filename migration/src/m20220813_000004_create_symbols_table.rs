use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS `symbols` (
            `name` varchar(30) NOT NULL PRIMARY KEY COMMENT '合約名稱',
            `alias` varchar(30) NOT NULL COMMENT '合約別名',
            `status` varchar(10) NOT NULL COMMENT '合約狀態 Trading Settling Closed',
            `base_currency` varchar(10) NOT NULL COMMENT '基礎貨幣',
            `quote_currency` varchar(10) NOT NULL COMMENT '報價貨幣',
            `price_scale` integer NOT NULL COMMENT '可以提交價格的小數位數',
            `taker_fee` varchar(30) NOT NULL COMMENT 'taker手續費',
            `maker_fee` varchar(30) NOT NULL COMMENT 'maker手續費',
            `funding_interval` integer NOT NULL COMMENT '資金費用結算週期',
            `max_trading_qty` double NOT NULL COMMENT '最大交易數量',
            `min_trading_qty` double NOT NULL COMMENT '最小交易數量',
            `qty_step` double NOT NULL COMMENT '合約數量最小單位',
            `post_only_max_trading_qty` varchar(30) NOT NULL COMMENT '訂單最大交易數量',
            `min_price` varchar(30) NOT NULL COMMENT '最小價格',
            `max_price` varchar(30) NOT NULL COMMENT '最大價格',
            `tick_size` varchar(30) NOT NULL COMMENT '價格最小增加或減少的數量',
            `min_leverage` integer NOT NULL COMMENT '最小槓桿',
            `max_leverage` integer NOT NULL COMMENT '最大槓桿',
            `leverage_step` varchar(30) NOT NULL COMMENT '槓桿最小增減單位'
        )"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE `symbols`";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
