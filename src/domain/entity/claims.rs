use axum::{async_trait, extract::FromRequestParts};
use axum_extra::extract::CookieJar;
use http::request::Parts;
use serde::{Deserialize, Serialize};

use crate::{
    domain::service::{error::ServiceError, util::token_service::TokenService},
    handlers::auth::COOKIE_KEY,
    infrastructure::service::token_service_impl::TokenServiceImpl,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub user_id: String,
    pub user_name: String,
    pub exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ServiceError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| ServiceError::MissingCredentials)?;
        let cookie = jar.get(COOKIE_KEY).map(|e| e.to_owned());

        let access_token = match &cookie {
            Some(cookie) => cookie.value(),
            None => return Err(ServiceError::MissingCredentials),
        };

        let claims = TokenServiceImpl
            .decode(access_token)
            .map_err(|_| ServiceError::InvalidToken)?;

        Ok(claims)
    }
}
