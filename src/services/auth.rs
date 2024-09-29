use axum::{
    async_trait,
    extract::FromRequestParts,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use http::{request::Parts, StatusCode};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::user_model::PubUserInfo;

static SECRETKEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

static COOKIEKEY: &str = "token";

pub fn authorize(
    AuthPayload {
        client_mail,
        client_secret,
    }: AuthPayload,
) -> Result<AccsessToken, AuthError> {
    if client_mail.is_empty() || client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // verify cliant secret and passeord
    let user_info: PubUserInfo;

    // create claims
    let claims = Claims::new(todo!());

    // create token
    let token = encode(&Header::default(), &claims, &SECRETKEYS.encoding).unwrap();

    Ok(AccsessToken::new(token))
}

// クライアントから送られる認証情報
#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_mail: String,
    client_secret: String,
}

// JsonWebTokenのPayload部
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    user_id: String,
    user_name: String,
    exp: usize,
}

impl Claims {
    pub fn new(user_info: PubUserInfo) -> Self {
        Self {
            iss: "http://localhost:8080".to_string(),
            sub: "AccessToken".to_string(),
            user_id: user_info.user_id,
            user_name: user_info.user_name,
            exp: 2000000000,
        }
    }
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
        
        let token_data =
        decode::<Claims>(access_token, &SECRETKEYS.decofing, &Validation::default()).unwrap();
        
        Ok(token_data.claims)
    }
}

// Base64エンコードされたJWTのラッパー
#[derive(Debug, Serialize)]
pub struct AccsessToken(String);

impl AccsessToken {
    pub fn new(access_token: String) -> Self {
        Self(access_token)
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

// Tokenをエンコード・デコードする際に必要なKeyのラッパ
// 実行時に一度だけ作られ、使いまわされる
struct Keys {
    encoding: EncodingKey,
    decofing: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decofing: DecodingKey::from_secret(secret),
        }
    }
}
