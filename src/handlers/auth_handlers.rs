use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use http::StatusCode;

use crate::{
    database::users_db::UserDb,
    models::auth_model::AuthPayload,
    services::auth::{AuthError, AuthorizeServices, COOKIEKEY}, util::ValidatedJson,
};

pub async fn login(
    State(db): State<UserDb>,
    jar: CookieJar,
    ValidatedJson(payload): ValidatedJson<AuthPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let services = AuthorizeServices::new(&db);

    let token = services.authorize(payload).await.map_err(|e| match e {
        AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let cookie = Cookie::new(COOKIEKEY, token.get().to_owned());
    Ok((StatusCode::OK, jar.add(cookie)))
}
