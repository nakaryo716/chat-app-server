use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use http::StatusCode;

use crate::{
    domain::{
        entity::{
            auth_payload::AuthPayload, claims::Claims, create_user_payload::CreateUserPayload,
            pub_user_info::PubUserInfo,
        },
        service::{
            auth_service::AuthorizeServices, error::ServiceError, user_service::UserService,
        },
    },
    infrastructure::{
        repository::user_repository_impl::UserRepositoryImpl,
        service::{
            password_hash_service_impl::PasswordHashServiceImpl,
            token_service_impl::TokenServiceImpl, uuid_gen_impl::UUIDGenIMpl,
        },
    },
    util::ValidatedJson,
    UserDb,
};

use super::auth::COOKIE_KEY;

pub async fn add_new_user(
    State(db): State<UserDb>,
    ValidatedJson(new_user_payload): ValidatedJson<CreateUserPayload>,
) -> Result<impl IntoResponse, ServiceError> {
    let service = UserService::new(
        UserRepositoryImpl::new(&db.pool),
        PasswordHashServiceImpl,
        UUIDGenIMpl,
    );
    let user_info = service.create_new_user(new_user_payload).await?;
    Ok((StatusCode::OK, Json(user_info)))
}

pub async fn get_user_info_handle(
    claims: Claims,
    State(db): State<UserDb>,
) -> Result<impl IntoResponse, ServiceError> {
    let user_service = UserService::new(
        UserRepositoryImpl::new(&db.pool),
        PasswordHashServiceImpl,
        UUIDGenIMpl,
    );
    let user_info = PubUserInfo {
        user_id: claims.user_id,
        user_name: claims.user_name,
    };
    let user_id = user_info.user_id;
    let query_res = user_service.get_user_by_id(user_id.to_string()).await?;
    Ok((StatusCode::OK, Json(query_res)))
}

pub async fn delete_user_handle(
    claims: Claims,
    jar: CookieJar,
    State(db): State<UserDb>,
    ValidatedJson(auth_payload): ValidatedJson<AuthPayload>,
) -> Result<impl IntoResponse, ServiceError> {
    // ユーザーデータの削除には再認証が必要
    let auth_service = AuthorizeServices::new(
        UserRepositoryImpl::new(&db.pool),
        TokenServiceImpl,
        PasswordHashServiceImpl,
    );
    auth_service.authorize(auth_payload).await?;

    let user_service = UserService::new(
        UserRepositoryImpl::new(&db.pool),
        PasswordHashServiceImpl,
        UUIDGenIMpl,
    );
    let user_info = PubUserInfo {
        user_id: claims.user_id,
        user_name: claims.user_name,
    };

    user_service.delete_user(&user_info.user_id).await?;

    // JWTが残らないようにCookieから削除
    Ok((StatusCode::NO_CONTENT, jar.remove(Cookie::from(COOKIE_KEY))))
}
