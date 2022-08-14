use super::jwt::Claims;
use bcrypt::{hash, verify, DEFAULT_COST};

use axum::{
    extract::Extension,
    // headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, MethodRouter},
    Json,
    Router,
};

use super::domain::{AuthBody, AuthPayload, CreateUser, UserInfo};
use super::usecase::UserUcase;
use pkg::responder::{failed, success, NoData, StatusCode as RespCode};

use std::sync::Arc;

/**
 * new handler
 */
pub fn new() -> Router {
    Router::new().merge(auth()).merge(info()).merge(create())
}

/**
 * generate route
 */
fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
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
    ) -> (StatusCode, Json<serde_json::Value>) {
        // Check if the user sent the credentials
        if payload.account.is_empty() || payload.password.is_empty() {
            let (_, resp) = failed(RespCode::STATUS_BADREQ, NoData::default());
            let jsonstr = serde_json::to_string_pretty(&resp).unwrap();
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!(jsonstr)));
        }

        //取得user data
        let user_data = c
            .user_ucase
            .get_by_account(payload.account.clone())
            .await
            .unwrap()
            .unwrap();

        //驗證密碼
        let valid = verify(payload.password, user_data.password.as_str()).unwrap();
        if !valid {
            let (_, resp) = failed(RespCode::STATUS_VALIDATION, NoData::default());
            let jsonstr = serde_json::to_string_pretty(&resp).unwrap();
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!(jsonstr)));
        }

        //產生jwt token
        let token = c
            .user_ucase
            .gen_token(user_data.account.clone(), user_data.role)
            .await
            .unwrap();

        c.user_ucase
            .save_token(user_data, token.clone())
            .await
            .unwrap();

        // Send the authorized token
        let (_, resp) = success(AuthBody::new(token));
        let jsonstr = serde_json::to_string_pretty(&resp).unwrap();
        (StatusCode::OK, Json(serde_json::json!(jsonstr)))
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
        let user_info = c.user_ucase.get_info(claims.account).await.unwrap();

        let (_, resp) = success(user_info);

        (StatusCode::OK, Json(resp))
    }

    route("/", get(get_info_handler))
}

fn create() -> Router {
    async fn create_user_handler(
        Json(mut payload): Json<CreateUser>,
        claims: Claims,
        Extension(c): Extension<Arc<UserContainer>>,
    ) -> (StatusCode, Json<serde_json::Value>) {
        //只有admin能新增用戶
        if claims.role < 99 {
            let (_, resp) = failed(RespCode::STATUS_VALIDATION, NoData::default());
            let jsonstr = serde_json::to_string_pretty(&resp).unwrap();

            return (StatusCode::BAD_REQUEST, Json(serde_json::json!(jsonstr)));
        }

        //判斷使用者存在
        let exist = c.user_ucase.is_exist(payload.account.clone()).await;
        if exist {
            let (_, resp) = failed(RespCode::STATUS_DUPLICATE, NoData::default());
            let jsonstr = serde_json::to_string_pretty(&resp).unwrap();
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!(jsonstr)));
        }

        //hash 密碼
        let pwd = hash(payload.password.clone(), DEFAULT_COST).unwrap();
        payload.password = pwd;

        //存入DB
        let user_data = c.user_ucase.create(payload).await.unwrap();
        let (_, resp) = success(UserInfo::from(user_data));

        let jsonstr = serde_json::to_string_pretty(&resp).unwrap();
        (StatusCode::OK, Json(serde_json::json!(jsonstr)))
    }

    route("/", post(create_user_handler))
}
