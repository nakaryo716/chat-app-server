use crate::{
    database::rooms_db::RoomDb,
    models::{auth_model::Claims, rooms_model::CreateRoom, user_model::PubUserInfo},
    services::rooms::RoomServices,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;

pub async fn create_room_handler(
    claims: Claims,
    State(room_db): State<RoomDb>,
    Json(payload): Json<CreateRoom>,
) -> Result<impl IntoResponse, StatusCode> {
    let room_services = RoomServices::new(&room_db);
    let user_info = PubUserInfo::from(claims);
    let room_info = room_services
        .create_room(payload, user_info)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(room_info)))
}

pub async fn get_specific_room_info(
    _claims: Claims,
    State(room_db): State<RoomDb>,
    Path(room_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let room_services = RoomServices::new(&room_db);
    let room_info = room_services
        .get_target_room_info(room_id.as_str())
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(room_info)))
}

pub async fn get_all_room_info_handler(
    _claims: Claims,
    State(room_db): State<RoomDb>,
) -> Result<impl IntoResponse, StatusCode> {
    let room_services = RoomServices::new(&room_db);
    let room_infos = room_services
        .get_all_room_info()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(room_infos)))
}

pub async fn delete_room_handler(
    claims: Claims,
    State(room_db): State<RoomDb>,
    Path(room_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let room_services = RoomServices::new(&room_db);
    let user_info = PubUserInfo::from(claims);
    room_services
        .delete_owner_room(room_id.as_str(), user_info)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
