//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "strategies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    pub symbol_name: String,
    pub state: i8,
    pub side: i8,
    #[sea_orm(column_name = "Remark")]
    pub remark: String,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    SignalRecords,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::SignalRecords => Entity::has_many(super::signal_records::Entity).into(),
        }
    }
}

impl Related<super::signal_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SignalRecords.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
