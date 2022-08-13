use super::jwt::{token_encode, AuthError, Claims};
use super::UserRepo;
use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, MethodRouter},
    Json, Router,
};
use chrono::NaiveDateTime;
use pkg::responder::{failed, success, Data, StatusCode as RespCode};
use serde::{Deserialize, Serialize};


use std::sync::Arc;

//request
#[derive(Deserialize)]
pub struct AuthPayload {
    pub account: String,
    pub password: String,
}

//response
#[derive(Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

impl AuthBody {
    //generate data
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: String::from("Bearer"),
        }
    }
}

pub struct UserContainer {
    user_repo: UserRepo,
}

impl UserContainer {
    pub fn new(user_repo: UserRepo) -> Self {
        UserContainer {
            user_repo: user_repo,
        }
    }
}

pub fn new() -> Router {
    Router::new().merge(auth()).merge(info())
}

/**
 * 登錄認證
 */
fn auth() -> Router {
    pub async fn login_handler(
        Json(payload): Json<AuthPayload>,
        Extension(c): Extension<Arc<UserContainer>>,
    ) -> Result<Json<AuthBody>, AuthError> {
        // Check if the user sent the credentials
        if payload.account.is_empty() || payload.password.is_empty() {
            return Err(AuthError::MissingCredentials);
        }

        let user_data = c
            .user_repo
            .get_by_account(payload.account.clone())
            .await
            .unwrap();

        let claims = Claims {
            account: user_data.account.clone(),
            role: user_data.role.clone(),
            // Mandatory expiry time as UTC timestamp
            exp: 2000000000, // May 2033
        };

        // Create the authorization token
        let token = token_encode(claims).map_err(|_| AuthError::TokenCreation)?;

        c.user_repo.save_token(user_data, token.clone()).await;

        // Send the authorized token
        Ok(Json(AuthBody::new(token)))
    }

    route("/login", post(login_handler))
}

/**
 * get user info
 */
fn info() -> Router {
    async fn get_info_handler(
        claims: Claims,
        Extension(c): Extension<Arc<UserContainer>>,
    ) -> impl IntoResponse {
        let user_data = c.user_repo.get_by_account(claims.account).await.unwrap();

        let info = UserInfo {
            account: user_data.account,
            name: user_data.name,
            role: user_data.role,
            state: user_data.state,
            created_at: user_data.created_at,
        };

        let (_, resp) = success(info);
        (StatusCode::OK, Json(resp))
    }

    route("/", get(get_info_handler))
}

#[derive(Serialize)]
struct UserInfo {
    account: String,
    name: String,
    role: i8,
    state: i8,
    created_at: NaiveDateTime,
}

impl Data for UserInfo {}

fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}

