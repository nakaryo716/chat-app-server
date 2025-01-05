use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use http::StatusCode;

pub static COOKIE_KEY: &str = "token";

use crate::{
    domain::{
        entity::auth_payload::AuthPayload,
        service::{auth_service::AuthorizeServices, error::ServiceError},
    },
    infrastructure::{
        repository::user_repository_impl::UserRepositoryImpl,
        service::{
            password_hash_service_impl::PasswordHashServiceImpl,
            token_service_impl::TokenServiceImpl,
        },
    },
    util::ValidatedJson,
    UserDb,
};

pub async fn login(
    State(db): State<UserDb>,
    jar: CookieJar,
    ValidatedJson(payload): ValidatedJson<AuthPayload>,
) -> Result<impl IntoResponse, ServiceError> {
    let services = AuthorizeServices::new(
        UserRepositoryImpl::new(&db.pool),
        TokenServiceImpl,
        PasswordHashServiceImpl,
    );
    let token = services.authorize(payload).await?;

    let cookie = Cookie::build((COOKIE_KEY, token.0))
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true);
    Ok((StatusCode::OK, jar.add(cookie)))
}
