use std::time::Duration;

use chrono::Local;
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
    user_id: String,
    user_name: String,
    exp: usize,
}

impl Claims {
    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }
}

impl From<PubUserInfo> for Claims {
    fn from(value: PubUserInfo) -> Self {
        // 現在時刻から1時間後をjwtの有効期限として設定している
        let offset_lim_time = Local::now() + Duration::new(3600, 0);
        let exp = offset_lim_time.timestamp() as usize;
        Self {
            user_id: value.get_user_id().to_owned(),
            user_name: value.get_user_name().to_owned(),
            exp,
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

    pub fn get(&self) -> &str {
        &self.0
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
