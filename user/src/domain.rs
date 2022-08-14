// use super::UserRepo;
// use sea_orm::{DatabaseConnection, DbErr};
// use std::sync::Arc;
// use axum::async_trait;
use anyhow::Result;
use async_trait::async_trait;
use entity::users::{ActiveModel as UserActiveModel, Model as UserModel};
use pkg::responder::Data;
use serde::{Deserialize, Serialize};
use std::convert::From;
use validator::Validate;

/**
 * Traits
 */
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_account(&self, account: String) -> Result<Option<UserModel>>;
    async fn save_token(&self, model: UserModel, token: String) -> Result<UserModel>;
    async fn is_exist(&self, account: String) -> bool;
    async fn create(&self, active: UserActiveModel) -> Result<UserModel>;
}

#[async_trait]
pub trait UserUsecase: Send + Sync {
    async fn get_by_account(&self, account: String) -> Result<Option<UserModel>>;
    async fn save_token(&self, model: UserModel, token: String) -> Result<UserModel>;
    async fn get_info(&self, account: String) -> Result<UserInfo>;
    async fn is_exist(&self, account: String) -> bool;
    async fn gen_token(&self, account: String, role: i8) -> Result<String>;
    async fn create(&self, body: CreateUser) -> Result<UserModel>;
}

/**
 * Create user request
 */
#[derive(Deserialize, Validate, Debug)]
pub struct CreateUser {
    #[validate(length(min = 4, max = 30))]
    pub account: String,
    #[validate(length(min = 6, max = 50))]
    pub password: String,
    #[validate(length(min = 1, max = 30))]
    pub name: String,
    #[validate(range(min = 1, max = 99))]
    pub role: i8,
}

/**
 * User info
 */
#[derive(Serialize)]
pub struct UserInfo {
    pub account: String,
    pub name: String,
    pub role: i8,
    pub state: i8,
    pub created_at: String,
}

impl Default for UserInfo {
    fn default() -> Self {
        UserInfo {
            account: "".to_owned(),
            name: "".to_owned(),
            role: 0,
            state: 0,
            created_at: "".to_owned(),
        }
    }
}

impl Data for UserInfo {}

impl From<UserModel> for UserInfo {
    fn from(model: UserModel) -> Self {
        let UserModel {
            account,
            name,
            role,
            state,
            created_at,
            ..
        } = model;

        UserInfo {
            account,
            name,
            role,
            state,
            created_at: created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

/**
 * Auth
 */
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct AuthPayload {
    #[validate(length(min = 4, max = 30))]
    pub account: String,
    #[validate(length(min = 6, max = 50))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

impl Data for AuthBody {}

impl AuthBody {
    //generate data
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: String::from("Bearer"),
        }
    }
}
