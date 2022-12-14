//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "subscribes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_account: String,
    pub strategy_name: String,
    pub amount: f64,
    pub is_isolated: Option<i8>,
    pub leverage: i16,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Orders,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Orders => Entity::has_many(super::orders::Entity).into(),
        }
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
