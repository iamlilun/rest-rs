use crate::domain::{AuthBody, AuthPayload, CreateUser, UserContainer, UserInfo};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use pkg::{
    jwt::Claims,
    responder::{failed, success, Detail, StatusCode as RespCode},
};
use std::sync::Arc;
use validator::Validate;

/**
 * 登錄認證
 */
pub async fn auth(
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

/**
 * get user info
 */
pub async fn get_info(
    claims: Claims,
    Extension(c): Extension<Arc<UserContainer>>,
) -> impl IntoResponse {
    let user_info = c.user_ucase.get_info(claims.account).await.unwrap();

    let (_, resp) = success(user_info);

    (StatusCode::OK, Json(resp))
}

/**
 * create user
 */
pub async fn create_user(
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
            Detail(String::from("Account already exist")),
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
