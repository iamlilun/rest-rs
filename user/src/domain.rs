// use super::UserRepo;
// use sea_orm::{DatabaseConnection, DbErr};
// use std::sync::Arc;
// use axum::async_trait;
use entity::users::Model as UserModel;
use pkg::responder::Data;
use serde::{Deserialize, Serialize};
use std::convert::From;

// #[async_trait]
// pub trait UserRepository {
//     async fn get_by_account(&mut self, account: String) -> Option<users::Model>;

//     async fn save_token(&mut self, model: users::Model, token: String) -> users::Model;
// }

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
#[derive(Deserialize)]
pub struct AuthPayload {
    pub account: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

// impl Data for AuthBody {}

impl AuthBody {
    //generate data
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: String::from("Bearer"),
        }
    }
}
