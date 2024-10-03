use axum::{
    async_trait,
    extract::FromRequestParts,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use http::{request::Parts, StatusCode};
use jsonwebtoken::{decode, encode, Header, Validation};
use once_cell::sync::Lazy;
use serde_json::json;

use crate::models::{
    auth_model::{AccsessToken, AuthPayload, Claims, Keys},
    user_model::PubUserInfo,
};

static SECRETKEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

static COOKIEKEY: &str = "token";

pub fn authorize(auth_payload: AuthPayload) -> Result<AccsessToken, AuthError> {
    if auth_payload.get_client_mail().is_empty() || auth_payload.get_client_secret().is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // verify cliant secret and passeord
    let user_info: PubUserInfo;

    // create claims
    // let claims = Claims::from(user_info);

    // create token
    // let token = encode(&Header::default(), &claims, SECRETKEYS.ref_encode_key()).unwrap();

    // Ok(AccsessToken::new(token))
    Ok(todo!())
}

// FromRequestPartsを実装することで各エンドポイントでCookieを取得し
// アクセストークンが有効かどうかを調べる
// 有効な場合はエンドポイントにトークンデータを渡して実行する
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();
        let cookie = jar.get(COOKIEKEY).map(|e| e.to_owned()).unwrap();
        let access_token = cookie.value();

        let token_data = decode::<Claims>(
            access_token,
            SECRETKEYS.ref_decode_key(),
            &Validation::default(),
        )
        .unwrap();

        Ok(token_data.claims)
    }
}

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
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
