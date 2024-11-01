use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use http::StatusCode;

use crate::{
    auth::{
        services::{AuthError, AuthorizeServices, COOKIEKEY},
        AuthPayload,
    },
    users::database::UserDb,
    util::ValidatedJson,
};

pub async fn login(
    State(db): State<UserDb>,
    jar: CookieJar,
    ValidatedJson(payload): ValidatedJson<AuthPayload>,
) -> Result<impl IntoResponse, AuthError> {
    let services = AuthorizeServices::new(&db);
    let token = services.authorize(payload).await?;

    let cookie = Cookie::build((COOKIEKEY, token.get().to_owned()))
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true);
    Ok((StatusCode::OK, jar.add(cookie)))
}
