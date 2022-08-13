use super::domain::UserInfo;
use super::repository::UserRepo;
use entity::users;

pub struct UserUcase {
    user_repo: UserRepo,
}

impl UserUcase {
    pub fn new(user_repo: UserRepo) -> Self {
        UserUcase { user_repo }
    }
}

impl UserUcase {
    pub async fn get_by_account(&self, account: String) -> Option<users::Model> {
        self.user_repo.get_by_account(account).await
    }

    pub async fn save_token(&self, model: users::Model, token: String) -> users::Model {
        self.user_repo.save_token(model, token).await
    }

    /**
     * 取得用戶資料
     */
    pub async fn get_info(&self, account: String) -> UserInfo {
        let user_data = self.get_by_account(account).await.unwrap();
        UserInfo::from(user_data)
    }

    /**
     * 取得用戶資料
     */
    pub async fn is_exist(&self, account: String) -> bool {
        self.user_repo.is_exist(account).await
    }
}
