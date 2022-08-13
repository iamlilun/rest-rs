use super::jwt::{token_encode, AuthError, Claims};

use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, MethodRouter},
    Json, Router,
};
use chrono::{Duration, Local, NaiveDateTime};

use super::domain::{AuthBody, AuthPayload, UserInfo};
use super::usecase::UserUcase;
use pkg::responder::{failed, success, StatusCode as RespCode};

use std::sync::Arc;

/**
 * new handler
 */
pub fn new() -> Router {
    Router::new().merge(auth()).merge(info())
}

/**
 * 要注入的容器
 */
pub struct UserContainer {
    user_ucase: UserUcase,
}

impl UserContainer {
    pub fn new(user_ucase: UserUcase) -> Self {
        UserContainer { user_ucase }
    }
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
            .user_ucase
            .get_by_account(payload.account.clone())
            .await
            .unwrap();

        //計算過期時間..
        let dt = Local::now() + Duration::weeks(1);
        let ts = dt.timestamp() as usize;

        let claims = Claims {
            account: user_data.account.clone(),
            role: user_data.role.clone(),
            // Mandatory expiry time as UTC timestamp
            exp: ts, //1 weeks
        };

        // Create the authorization token
        let token = token_encode(claims).map_err(|_| AuthError::TokenCreation)?;

        c.user_ucase.save_token(user_data, token.clone()).await;

        // Send the authorized token
        Ok(Json(AuthBody::new(token)))
    }

    route("/login", post(login_handler))
}

/**
 * generate route
 */
fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}

/**
 * get user info
 */
fn info() -> Router {
    async fn get_info_handler(
        claims: Claims,
        Extension(c): Extension<Arc<UserContainer>>,
    ) -> impl IntoResponse {
        let user_info = c.user_ucase.get_info(claims.account).await;

        let (_, resp) = success(user_info);

        (StatusCode::OK, Json(resp))
    }

    route("/", get(get_info_handler))
}
