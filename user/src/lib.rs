pub mod domain;
pub mod handler;
pub mod repository;
pub mod usecase;

pub use domain::*;
pub use handler::*;
pub use repository::*;

/**
 * JWT
 */
pub mod jwt {
    use axum::{
        async_trait,
        extract::{FromRequest, RequestParts, TypedHeader},
        headers::{authorization::Bearer, Authorization},
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };

    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
    use once_cell::sync::Lazy;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::default::Default;
    use std::fmt::Display;

    static KEYS: Lazy<Keys> = Lazy::new(|| {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Keys::new(secret.as_bytes())
    });

    /**
     * make token
     */
    pub fn encode_token(claims: Claims) -> jsonwebtoken::errors::Result<String> {
        encode(&Header::default(), &claims, &KEYS.encoding)
    }

    /**
     * decode token
     */
    pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
        decode::<Claims>(token, &KEYS.decoding, &Validation::default())
    }

    /**
     * Error handle
     */
    #[derive(Debug)]
    pub enum AuthError {
        WrongCredentials,
        MissingCredentials,
        TokenCreation,
        InvalidToken,
    }

    impl IntoResponse for AuthError {
        fn into_response(self) -> Response {
            let (status, error_message) = match self {
                AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
                AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
                AuthError::TokenCreation => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error")
                }
                AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            };
            let body = Json(json!({
                "error": error_message,
            }));
            (status, body).into_response()
        }
    }

    /**
     * Keys
     */

    pub struct Keys {
        encoding: EncodingKey,
        decoding: DecodingKey,
    }

    impl Keys {
        //encode & decode
        pub fn new(secret: &[u8]) -> Self {
            Self {
                encoding: EncodingKey::from_secret(secret),
                decoding: DecodingKey::from_secret(secret),
            }
        }
    }

    /**
     * Claims
     */
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        pub account: String, //user account
        pub role: i8,        //user role
        pub exp: usize,      //ExpiresAt
    }

    #[async_trait]
    impl<B> FromRequest<B> for Claims
    where
        B: Send,
    {
        type Rejection = AuthError;

        async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
            // Extract the token from the authorization header
            let TypedHeader(Authorization(bearer)) =
                TypedHeader::<Authorization<Bearer>>::from_request(req)
                    .await
                    .map_err(|_| AuthError::InvalidToken)?;

            // Decode the user data
            let token_data = decode_token(bearer.token()).map_err(|_| AuthError::InvalidToken)?;

            Ok(token_data.claims)
        }
    }

    impl Display for Claims {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Account: {}\nRole: {}", self.account, self.role)
        }
    }
}
