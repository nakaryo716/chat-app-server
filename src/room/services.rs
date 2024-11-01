use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde_json::json;
use tokio::sync::broadcast::Sender;

use crate::{auth::Claims, users::PubUserInfo};

use super::{
    database::{RoomError, RoomManage},
    CreateRoom, Room, RoomInfo,
};

pub struct RoomServices<'a, T>
where
    T: RoomManage,
{
    room_db: &'a T,
}

impl<'a, T> RoomServices<'a, T>
where
    T: RoomManage<Info = RoomInfo, Data = Room, Error = RoomError>,
{
    pub fn new(room_db: &'a T) -> Self {
        Self { room_db }
    }

    pub fn create_room(
        &self,
        payload: CreateRoom,
        user_info: PubUserInfo,
    ) -> Result<RoomInfo, RoomError> {
        let room_info = self
            .room_db
            .open_new_room(payload.get_room_name(), user_info)
            .map_err(|e| e)?;

        Ok(room_info)
    }

    pub fn get_target_room_info(&self, room_id: &str) -> Result<RoomInfo, RoomError> {
        let room = self.room_db.listen_room(room_id).map_err(|e| e)?;
        Ok(room.get_room_info().to_owned())
    }

    pub fn get_owner_room_info(&self, claims: Claims) -> Result<Vec<RoomInfo>, RoomError> {
        let room_owner_id = claims.get_user_id();
        let rooms = self.room_db.get_owner_rooms(room_owner_id).map_err(|e| e)?;
        Ok(rooms)
    }

    pub fn get_all_room_info(&self) -> Result<Vec<RoomInfo>, RoomError> {
        let rooms = self.room_db.get_all_room().map_err(|e| e)?;
        Ok(rooms)
    }

    pub fn get_sender(&self, room_id: &str) -> Result<Sender<String>, RoomError> {
        let room = self.room_db.listen_room(room_id).map_err(|e| e)?;
        Ok(room.get_sender())
    }

    pub fn delete_owner_room(
        &self,
        room_id: &str,
        user_info: PubUserInfo,
    ) -> Result<(), RoomError> {
        let room = self.room_db.listen_room(room_id).map_err(|e| e)?;
        let room_owner = room.get_room_info().get_created_by_id();

        if room_owner == user_info.get_user_id() {
            self.room_db.delete_room(room_id).map_err(|e| e)
        } else {
            Err(RoomError::DbError)
        }
    }
}

impl IntoResponse for RoomError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            RoomError::DbError => (StatusCode::INTERNAL_SERVER_ERROR, "Server error occurred"),
            RoomError::IdNotFound => (StatusCode::NOT_FOUND, "Room not found"),
        };

        let body = Json(json!({
            "error": message,
        }));
        (status, body).into_response()
    }
}
