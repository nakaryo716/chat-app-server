use argon2::{Argon2, PasswordHash, PasswordVerifier};
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

use crate::{
    database::users_db::UserDataViewer,
    models::{
        auth_model::{AccsessToken, AuthPayload, Claims, Keys},
        user_model::{PubUserInfo, User},
    },
};

static SECRETKEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub static COOKIEKEY: &str = "token";

pub struct AuthorizeServices<'a, T>
where
    T: UserDataViewer<String, String, Error = sqlx::error::Error>,
{
    db_pool: &'a T,
}

impl<'a, T> AuthorizeServices<'a, T>
where
    T: UserDataViewer<
        String,
        String,
        FullUserData = User,
        UserInfo = PubUserInfo,
        Error = sqlx::error::Error,
    >,
{
    pub fn new(db_pool: &'a T) -> Self {
        Self { db_pool }
    }

    pub async fn authorize(&self, auth_payload: AuthPayload) -> Result<AccsessToken, AuthError> {
        if auth_payload.get_client_mail().is_empty() || auth_payload.get_client_secret().is_empty()
        {
            return Err(AuthError::MissingCredentials);
        }
        // verify cliant secret and passeord
        let full_user_data = self
            .db_pool
            .get_user_data(auth_payload.get_client_mail().to_string())
            .await
            .map_err(|e| AuthError::from(e))?;

        // ハッシュ化されたデータとペイロードでverifyする
        // CPUバウンドのためブロッキングスレッドで行っている
        let full_data = full_user_data.clone();
        let res = tokio::task::spawn_blocking(move || {
            AuthorizeServices::<T>::verify_pass(
                full_data.get_user_pass(),
                auth_payload.get_client_secret(),
            )
        })
        .await
        .map_err(|_| AuthError::Server)?;

        if res.is_err() {
            return Err(AuthError::WrongCredentials);
        }

        // ユーザー認証が完了したらUserデータからPubUserInfoを作成
        let user_info = PubUserInfo::from(full_user_data);
        // create claims
        let claims = Claims::from(user_info);

        // create token
        let token = encode(&Header::default(), &claims, SECRETKEYS.ref_encode_key())
            .map_err(|_| AuthError::TokenCreation)?;

        Ok(AccsessToken::new(token))
    }

    fn verify_pass(hashed_pass: &str, payload_pass: &str) -> Result<(), AuthError> {
        let pwd_hash = PasswordHash::new(hashed_pass).map_err(|_| AuthError::Server)?;
        Argon2::default()
            .verify_password(payload_pass.as_bytes(), &pwd_hash)
            .map_err(|_| AuthError::WrongCredentials)
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
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| AuthError::MissingCredentials)?;
        let cookie = jar.get(COOKIEKEY).map(|e| e.to_owned());

        let access_token = match &cookie {
            Some(cookie) => cookie.value(),
            None => return Err(AuthError::MissingCredentials),
        };

        let token_data = decode::<Claims>(
            access_token.as_ref(),
            SECRETKEYS.ref_decode_key(),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    WrongCredentials,
    TokenCreation,
    InvalidToken,
    UserNotFound,
    Server,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthError::Server => (StatusCode::INTERNAL_SERVER_ERROR, "Server error occurred"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl From<sqlx::error::Error> for AuthError {
    fn from(value: sqlx::error::Error) -> Self {
        match value {
            sqlx::error::Error::RowNotFound => AuthError::UserNotFound,
            _ => AuthError::Server,
        }
    }
}
