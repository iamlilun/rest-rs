// use super::domain::UserRepository;
use entity::{orders, prelude::*, users};
use sea_orm::{
    prelude::*, ActiveValue::NotSet, ConnectionTrait, Database, DatabaseConnection, DbBackend,
    DbErr, QueryOrder, Set, Statement,
};
use std::sync::Arc;

pub struct UserRepo {
    db: DatabaseConnection,
}

impl UserRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        UserRepo { db: db }
    }
}

impl UserRepo {
    pub async fn get_by_account(&self, account: String) -> Option<users::Model> {
        Users::find()
            .filter(users::Column::Account.eq(account))
            .one(&self.db)
            .await
            .unwrap()
    }

    pub async fn save_token(&self, model: users::Model, token: String) -> users::Model {
        let mut user: entity::users::ActiveModel = model.into();
        user.token = Set(token);
        user.update(&self.db).await.unwrap()
    }
}
