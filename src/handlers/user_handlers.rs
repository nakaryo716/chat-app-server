use crate::{
    database::users_db::UserDb,
    models::{
        auth_model::{AuthPayload, Claims},
        user_model::{CreateUserPayload, PubUserInfo},
    },
    services::{
        auth::{AuthorizeServices, COOKIEKEY},
        user::{UserServices, UserServiciesError},
    }, util::ValidatedJson,
};
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use http::StatusCode;

pub async fn add_new_user(
    State(db): State<UserDb>,
    ValidatedJson(new_user_payload): ValidatedJson<CreateUserPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let service = UserServices::new(&db);
    let user_info = service
        .create_new_user(new_user_payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(user_info)))
}

pub async fn get_user_info_handle(
    claims: Claims,
    State(db): State<UserDb>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_service = UserServices::new(&db);

    let user_info = PubUserInfo::from(claims);
    let user_id = user_info.get_user_id();

    let query_res = user_service
        .get_user_by_id(user_id.to_string())
        .await
        .map_err(|e| match e {
            UserServiciesError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok((StatusCode::OK, Json(query_res)))
}

pub async fn delete_user_handle(
    claims: Claims,
    jar: CookieJar,
    State(db): State<UserDb>,
    ValidatedJson(auth_payload): ValidatedJson<AuthPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    // ユーザーデータの削除には再認証が必要
    let auth_service = AuthorizeServices::new(&db);
    auth_service
        .authorize(auth_payload)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_service = UserServices::new(&db);
    let user_info = PubUserInfo::from(claims);

    user_service
        .delete_user(user_info.get_user_id())
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // JWTが残らないようにCookieから削除
    Ok(jar.remove(Cookie::from(COOKIEKEY)))
}
