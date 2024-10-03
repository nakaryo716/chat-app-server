use crate::{
    database::rooms_db::{RoomError, RoomManage},
    models::{
        rooms_model::{CreateRoom, Room, RoomInfo},
        user_model::PubUserInfo,
    },
};
use tokio::sync::broadcast::Sender;

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
    pub fn create_room(
        &self,
        payload: CreateRoom,
        user_info: PubUserInfo,
    ) -> Result<RoomInfo, RoomError> {
        // TODO: ルーム名のバリデーションを行う
        let room_info = self
            .room_db
            .open_new_room(payload.get_room_name(), user_info.get_user_id())
            .map_err(|e| e)?;

        Ok(room_info)
    }

    pub fn get_target_room_info(&self, room_id: &str) -> Result<RoomInfo, RoomError> {
        let room = self.room_db.listen_room(room_id).map_err(|e| e)?;
        Ok(room.get_room_info().to_owned())
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
        let room_owner = room.get_room_info().get_created_by();

        if room_owner == user_info.get_user_id() {
            self.room_db.delete_room(room_id).map_err(|e| e)
        } else {
            Err(RoomError::DbError)
        }
    }
}
