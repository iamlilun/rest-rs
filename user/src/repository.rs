use entity::{prelude::*, users};
use sea_orm::{prelude::*, ConnectionTrait, DatabaseConnection, DbBackend, Set, Statement};

pub struct UserRepo {
    db: DatabaseConnection,
}

impl UserRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        UserRepo { db: db }
    }
}

impl UserRepo {
    pub async fn get_by_account(&self, account: String) -> anyhow::Result<Option<users::Model>> {
        let model = Users::find()
            .filter(users::Column::Account.eq(account))
            .one(&self.db)
            .await?;

        Ok(model)
    }

    pub async fn save_token(
        &self,
        model: users::Model,
        token: String,
    ) -> anyhow::Result<users::Model> {
        let mut user: entity::users::ActiveModel = model.into();
        user.token = Set(token);
        let res = user.update(&self.db).await?;
        Ok(res)
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

    pub async fn create(&self, active: users::ActiveModel) -> anyhow::Result<users::Model> {
        let model = active.insert(&self.db).await?;
        Ok(model)
    }
}
