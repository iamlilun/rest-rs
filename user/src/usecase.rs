use super::domain::{CreateUser, UserInfo};
use super::jwt::{encode_token, Claims};
use super::repository::UserRepo;
use chrono::{Duration, Local};
use entity::users;
use sea_orm::ActiveValue::Set;

pub struct UserUcase {
    user_repo: UserRepo,
}

impl UserUcase {
    pub fn new(user_repo: UserRepo) -> Self {
        UserUcase { user_repo }
    }
}

impl UserUcase {
    pub async fn get_by_account(&self, account: String) -> anyhow::Result<Option<users::Model>> {
        let res = self.user_repo.get_by_account(account).await?;
        Ok(res)
    }

    /**
     * 儲存用戶token
     */
    pub async fn save_token(
        &self,
        model: users::Model,
        token: String,
    ) -> anyhow::Result<users::Model> {
        let res = self.user_repo.save_token(model, token).await?;
        Ok(res)
    }

    /**
     * 取得用戶資料
     */
    pub async fn get_info(&self, account: String) -> anyhow::Result<UserInfo> {
        let user_data = self.get_by_account(account).await?.unwrap();
        let info = UserInfo::from(user_data);
        Ok(info)
    }

    /**
     * 取得用戶資料
     */
    pub async fn is_exist(&self, account: String) -> bool {
        self.user_repo.is_exist(account).await
    }

    /**
     * 產生jwt token
     */
    pub async fn gen_token(&self, account: String, role: i8) -> anyhow::Result<String> {
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
    pub async fn create(&self, body: CreateUser) -> anyhow::Result<users::Model> {
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
