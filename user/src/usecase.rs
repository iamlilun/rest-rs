use super::domain::{CreateUser, UserInfo, UserRepository, UserUsecase};
use super::jwt::{encode_token, Claims};
use async_trait::async_trait;
use chrono::{Duration, Local};
use entity::users;
use sea_orm::ActiveValue::Set;
use std::sync::Arc;

pub struct UserUcase {
    user_repo: Arc<dyn UserRepository>,
}

impl UserUcase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Arc<dyn UserUsecase> {
        Arc::new(UserUcase { user_repo })
    }
}

#[async_trait]
impl UserUsecase for UserUcase {
    async fn get_by_account(&self, account: String) -> anyhow::Result<Option<users::Model>> {
        let res = self.user_repo.get_by_account(account).await?;
        Ok(res)
    }

    /**
     * 儲存用戶token
     */
    async fn save_token(&self, model: users::Model, token: String) -> anyhow::Result<users::Model> {
        let res = self.user_repo.save_token(model, token).await?;
        Ok(res)
    }

    /**
     * 取得用戶資料
     */
    async fn get_info(&self, account: String) -> anyhow::Result<UserInfo> {
        let user_data = self.get_by_account(account).await?.unwrap();
        let info = UserInfo::from(user_data);
        Ok(info)
    }

    /**
     * 取得用戶資料
     */
    async fn is_exist(&self, account: String) -> bool {
        self.user_repo.is_exist(account).await
    }

    /**
     * 產生jwt token
     */
    async fn gen_token(&self, account: String, role: i8) -> anyhow::Result<String> {
        //計算過期時間..
        let dt = Local::now() + Duration::weeks(1);
        let ts = dt.timestamp() as usize;

        let claims = Claims {
            account: account,
            role: role,
            // Mandatory expiry time as UTC timestamp
            exp: ts, //1 weeks
        };

        // Create the authorization token
        let token = encode_token(claims)?;

        Ok(token)
    }

    /**
     * 建立用戶
     */
    async fn create(&self, body: CreateUser) -> anyhow::Result<users::Model> {
        let active_model = users::ActiveModel {
            account: Set(body.account),
            password: Set(body.password),
            name: Set(body.name),
            role: Set(body.role),
            ..Default::default()
        };
        let res = self.user_repo.create(active_model).await?;
        Ok(res)
    }
}
