use super::domain::UserRepository;
use async_trait::async_trait;
use entity::{prelude::*, users};
use pkg::db::ORM;
use sea_orm::{prelude::*, ConnectionTrait, DbBackend, Set, Statement};
use std::sync::Arc;

pub struct UserRepo {
    mysql: Arc<dyn ORM>,
}

#[async_trait]
impl UserRepository for UserRepo {
    async fn get_by_account(&self, account: String) -> anyhow::Result<Option<users::Model>> {
        let db = self.mysql.get_db().await;
        let model = Users::find()
            .filter(users::Column::Account.eq(account))
            .one(db)
            .await?;

        Ok(model)
    }

    async fn save_token(&self, model: users::Model, token: String) -> anyhow::Result<users::Model> {
        let db = self.mysql.get_db().await;
        let mut user: entity::users::ActiveModel = model.into();
        user.token = Set(token);
        let res = user.update(db).await?;
        Ok(res)
    }

    async fn is_exist(&self, account: String) -> bool {
        let db = self.mysql.get_db().await;
        let res = db
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

    async fn create(&self, active: users::ActiveModel) -> anyhow::Result<users::Model> {
        let db = self.mysql.get_db().await;
        let model = active.insert(db).await?;
        Ok(model)
    }
}

impl UserRepo {
    pub fn new(mysql: Arc<dyn ORM>) -> Arc<dyn UserRepository> {
        Arc::new(UserRepo { mysql })
    }
}
