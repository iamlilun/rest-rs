// use super::UserRepo;
use sea_orm::{DatabaseConnection, DbErr};
// use std::sync::Arc;
use axum::async_trait;
use entity::users;

#[async_trait]
pub trait UserRepository {
    async fn get_by_account(&mut self, account: String) -> Option<users::Model>;

    async fn save_token(&mut self, model: users::Model, token: String) -> users::Model;
}
