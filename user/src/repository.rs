// use super::domain::UserRepository;
use async_trait::async_trait;
use entity::{orders, prelude::*, users};
use sea_orm::{
    prelude::*, query, ActiveValue::NotSet, ConnectionTrait, Database, DatabaseConnection,
    DbBackend, DbErr, QueryOrder, QueryResult, Set, Statement,
};
use std::sync::Arc;

#[async_trait]
pub trait UserRepository {
    async fn get_by_account(&mut self, account: String) -> Option<users::Model>;

    async fn save_token(&mut self, model: users::Model, token: String) -> users::Model;
}

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

    pub async fn is_exist(&self, account: String) -> bool {
        let res = self
            .db
            .query_one(Statement::from_sql_and_values(
                DbBackend::MySql,
                r#"SELECT COUNT(account) AS count FROM users WHERE account = ?"#,
                vec![account.into()],
            ))
            .await
            .unwrap();

        let count: i64 = res.unwrap().try_get("", "count").unwrap();

        count > 0
    }
}
