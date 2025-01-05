use tokio::sync::broadcast::Sender;

use crate::domain::{
    entity::{
        claims::Claims, create_room::CreateRoom, pub_user_info::PubUserInfo, room_info::RoomInfo,
    },
    repository::room_repository::RoomRepository,
};

use super::error::ServiceError;

pub struct RoomServices<R>
where
    R: RoomRepository,
{
    repo: R,
}

impl<R> RoomServices<R>
where
    R: RoomRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn create_room(
        &self,
        payload: CreateRoom,
        user_info: PubUserInfo,
    ) -> Result<RoomInfo, ServiceError> {
        let room_info = self.repo.open_new_room(&payload.room_name, &user_info)?;

        Ok(room_info)
    }

    pub fn get_target_room_info(&self, room_id: &str) -> Result<RoomInfo, ServiceError> {
        let room = self.repo.listen_room(room_id)?;
        Ok(room.room_info.to_owned())
    }

    pub fn get_owner_room_info(&self, claims: Claims) -> Result<Vec<RoomInfo>, ServiceError> {
        let room_owner_id = &claims.user_id;
        let rooms = self.repo.get_owner_rooms(room_owner_id)?;
        Ok(rooms)
    }

    pub fn get_all_room_info(&self) -> Result<Vec<RoomInfo>, ServiceError> {
        let rooms = self.repo.get_all_room()?;
        Ok(rooms)
    }

    pub fn get_sender(&self, room_id: &str) -> Result<Sender<String>, ServiceError> {
        let room = self.repo.listen_room(room_id)?;
        Ok(room.sender)
    }

    pub fn delete_owner_room(
        &self,
        room_id: &str,
        user_info: PubUserInfo,
    ) -> Result<(), ServiceError> {
        let room = self.repo.listen_room(room_id).map_err(|e| e)?;
        let room_owner = room.room_info.created_by_id;

        if room_owner == user_info.user_id {
            self.repo.delete_room(room_id)?;
        } else {
            return Err(ServiceError::NotFound);
        }
        Ok(())
    }
}
