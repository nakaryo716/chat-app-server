use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

use super::user_model::PubUserInfo;

// クライアントから送られる認証情報
#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_mail: String,
    client_secret: String,
}

impl AuthPayload {
    pub fn get_client_mail(&self) -> &str {
        &self.client_mail
    }

    pub fn get_client_secret(&self) -> &str {
        &self.client_secret
    }
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

impl From<PubUserInfo> for Claims {
    fn from(value: PubUserInfo) -> Self {
        Self {
            iss: "http://localhost:8080".to_string(),
            sub: "AccessToken".to_string(),
            user_id: value.get_user_id().to_owned(),
            user_name: value.get_user_name().to_owned(),
            exp: 2000000000,
        }
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

// Tokenをエンコード・デコードする際に必要なKeyのラッパ
// 実行時に一度だけ作られ、使いまわされる
pub struct Keys {
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

    pub fn ref_encode_key(&self) -> &EncodingKey {
        &self.encoding
    }

    pub fn ref_decode_key(&self) -> &DecodingKey {
        &self.decofing
    }
}
