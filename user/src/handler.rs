use super::domain::{AuthBody, AuthPayload, CreateUser, UserInfo, UserUsecase};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, MethodRouter},
    Json, Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use pkg::{
    jwt::Claims,
    responder::{failed, success, Detail, StatusCode as RespCode},
};
use std::sync::Arc;
use validator::Validate;

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
 * Extension container
 */
pub struct UserContainer {
    user_ucase: Arc<dyn UserUsecase>,
}

impl UserContainer {
    pub fn new(user_ucase: Arc<dyn UserUsecase>) -> Arc<UserContainer> {
        Arc::new(UserContainer { user_ucase })
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
        //request validate
        match payload.validate() {
            Ok(_) => (),
            Err(e) => {
                println!("{:#?}", e);
                let (_, resp) = failed(RespCode::StatusBadReq, Detail("validate error".to_owned()));
                let jsonv = serde_json::to_value(resp).unwrap();
                return (StatusCode::BAD_REQUEST, Json(jsonv));
            }
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
            let (_, resp) = failed(
                RespCode::StatusValidation,
                Detail("Password Verify error".to_owned()),
            );
            let jsonv = serde_json::to_value(resp).unwrap();
            return (StatusCode::BAD_REQUEST, Json(jsonv));
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
        let jsonv = serde_json::to_value(resp).unwrap();
        (StatusCode::OK, Json(jsonv))
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
            let (_, resp) = failed(
                RespCode::StatusValidation,
                Detail("Permission error".to_owned()),
            );
            let jsonv = serde_json::to_value(resp).unwrap();

            return (StatusCode::BAD_REQUEST, Json(serde_json::json!(jsonv)));
        }

        //request validate
        match payload.validate() {
            Ok(_) => (),
            Err(e) => {
                println!("{:#?}", e);
                let (_, resp) = failed(RespCode::StatusBadReq, Detail("validate error".to_owned()));
                let jsonv = serde_json::to_value(resp).unwrap();
                return (StatusCode::BAD_REQUEST, Json(jsonv));
            }
        }

        //判斷使用者存在
        let exist = c.user_ucase.is_exist(payload.account.clone()).await;
        if exist {
            let (_, resp) = failed(
                RespCode::StatusDuplicate,
                Detail(String::from("Account not exist")),
            );
            let jsonv = serde_json::to_value(resp).unwrap();
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!(jsonv)));
        }

        //hash 密碼
        let pwd = hash(payload.password.clone(), DEFAULT_COST).unwrap();
        payload.password = pwd;

        //存入DB
        let user_data = c.user_ucase.create(payload).await.unwrap();
        let (_, resp) = success(UserInfo::from(user_data));

        let jsonv = serde_json::to_value(resp).unwrap();
        (StatusCode::OK, Json(serde_json::json!(jsonv)))
    }

    route("/", post(create_user_handler))
}
