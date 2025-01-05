use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;

use crate::{
    domain::{
        entity::{claims::Claims, create_room::CreateRoom, pub_user_info::PubUserInfo},
        service::{error::ServiceError, room_service::RoomServices},
    },
    infrastructure::repository::room_repository_impl::RoomRepositoryImpl,
    util::ValidatedJson,
    RoomDb,
};

pub async fn create_room_handler(
    claims: Claims,
    State(room_db): State<RoomDb>,
    ValidatedJson(payload): ValidatedJson<CreateRoom>,
) -> Result<impl IntoResponse, ServiceError> {
    let room_services = RoomServices::new(RoomRepositoryImpl::new(room_db));
    let user_info = PubUserInfo {
        user_id: claims.user_id,
        user_name: claims.user_name,
    };
    let room_info = room_services.create_room(payload, user_info)?;
    Ok((StatusCode::OK, Json(room_info)))
}

pub async fn get_specific_room_info(
    _claims: Claims,
    State(room_db): State<RoomDb>,
    Path(room_id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let room_services = RoomServices::new(RoomRepositoryImpl::new(room_db));
    let room_info = room_services.get_target_room_info(room_id.as_str())?;
    Ok((StatusCode::OK, Json(room_info)))
}

pub async fn get_owner_room_handler(
    claims: Claims,
    State(room_db): State<RoomDb>,
) -> Result<impl IntoResponse, ServiceError> {
    let room_services = RoomServices::new(RoomRepositoryImpl::new(room_db));

    let owner_room_info = room_services.get_owner_room_info(claims).map_err(|e| e)?;
    Ok((StatusCode::OK, Json(owner_room_info)))
}

pub async fn get_all_room_info_handler(
    _claims: Claims,
    State(room_db): State<RoomDb>,
) -> Result<impl IntoResponse, ServiceError> {
    let room_services = RoomServices::new(RoomRepositoryImpl::new(room_db));
    let room_infos = room_services.get_all_room_info()?;
    Ok((StatusCode::OK, Json(room_infos)))
}

pub async fn delete_room_handler(
    claims: Claims,
    State(room_db): State<RoomDb>,
    Path(room_id): Path<String>,
) -> Result<impl IntoResponse, ServiceError> {
    let room_services = RoomServices::new(RoomRepositoryImpl::new(room_db));
    let user_info = PubUserInfo {
        user_id: claims.user_id,
        user_name: claims.user_name,
    };
    room_services.delete_owner_room(room_id.as_str(), user_info)?;
    Ok(StatusCode::NO_CONTENT)
}
