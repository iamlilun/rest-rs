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

pub struct UserHaandler {
    user_repo: UserRepo,
}

impl UserHaandler {
    pub fn new(user_repo: UserRepo) -> Self {
        UserHaandler {
            user_repo: user_repo,
        }
    }
}

pub fn auth() -> Router {
    pub async fn authorize(
        Json(payload): Json<AuthPayload>,
        Extension(handler): Extension<Arc<UserHaandler>>,
    ) -> Result<Json<AuthBody>, AuthError> {
        // Check if the user sent the credentials
        if payload.account.is_empty() || payload.password.is_empty() {
            return Err(AuthError::MissingCredentials);
        }
        let user_data = handler
            .user_repo
            .get_by_account(payload.account.clone())
            .await
            .unwrap()
            .unwrap();
        let claims = Claims {
            account: user_data.account.clone(),
            role: user_data.role.clone(),
            // Mandatory expiry time as UTC timestamp
            exp: 2000000000, // May 2033
        };
        // Create the authorization token
        let token = token_encode(claims).map_err(|_| AuthError::TokenCreation)?;

        handler
            .user_repo
            .save_token(user_data, token.clone())
            .await
            .map_err(|_| AuthError::WrongCredentials)?;
        // Send the authorized token
        Ok(Json(AuthBody::new(token)))
    }

    pub async fn protected(claims: Claims) -> Result<String, AuthError> {
        // Send the protected data to the user
        Ok(format!(
            "Welcome to the protected area :)\nYour data:\n{}",
            claims
        ))
    }

    route("/login", post(authorize)).route("/protected", get(protected))
}

// fn get_info() -> Router {
//     async fn handler() -> impl IntoResponse {}

//     route("", get(handler))
// }

fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}