use std::time::Duration;

use chrono::Local;
use validator::Validate;

use crate::domain::{
    entity::{access_token::AccessToken, auth_payload::AuthPayload, claims::Claims},
    repository::user_repository::UserRepository,
};

use super::{
    error::ServiceError,
    util::{password_hash_service::PasswordHashService, token_service::TokenService},
};

pub struct AuthorizeServices<R, T, V>
where
    R: UserRepository,
    T: TokenService,
    V: PasswordHashService,
{
    repo: R,
    token_service: T,
    verifier: V,
}

impl<R, T, V> AuthorizeServices<R, T, V>
where
    R: UserRepository,
    T: TokenService,
    V: PasswordHashService,
{
    pub fn new(repo: R, token_service: T, verifier: V) -> Self {
        Self {
            repo,
            token_service,
            verifier,
        }
    }

    pub async fn authorize(&self, auth_payload: AuthPayload) -> Result<AccessToken, ServiceError> {
        if let Err(_) = auth_payload.validate() {
            return Err(ServiceError::Validation);
        }

        let full_user_data = self.repo.get_user_data(&auth_payload.client_mail).await?;

        // ハッシュ化されたデータとペイロードでverifyする
        // CPUバウンドのためブロッキングスレッドで行っている
        let verify_result = self
            .verifier
            .verify_pwd(&auth_payload.client_pass, &full_user_data.user_pass)?;

        if !verify_result {
            return Err(ServiceError::WrongCredentials);
        }

        let offset_lim_time = Local::now() + Duration::new(3600, 0);
        let exp = offset_lim_time.timestamp() as usize;

        // create claims
        let claims = Claims {
            user_id: full_user_data.user_id,
            user_name: full_user_data.user_name,
            exp,
        };
        // create token
        let token = self.token_service.encode(&claims)?;
        Ok(AccessToken(token))
    }
}
