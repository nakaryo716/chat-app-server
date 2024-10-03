use crate::{
    database::users_db::UserDb, models::user_model::CreateUserPayload, services::user::UserServices,
};
use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;

pub async fn add_new_user(
    State(db): State<UserDb>,
    Json(new_user_payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let service = UserServices::new(&db);
    let user_info = service
        .create_new_user(new_user_payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(user_info)))
}
