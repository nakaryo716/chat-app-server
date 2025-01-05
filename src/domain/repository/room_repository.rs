use crate::domain::entity::{pub_user_info::PubUserInfo, room::Room, room_info::RoomInfo};

use super::error::RepositoryError;

pub trait RoomRepository {
    fn open_new_room(
        &self,
        room_name: &str,
        user_info: &PubUserInfo,
    ) -> Result<RoomInfo, RepositoryError>;
    fn listen_room(&self, room_id: &str) -> Result<Room, RepositoryError>;
    fn get_owner_rooms(&self, owner_id: &str) -> Result<Vec<RoomInfo>, RepositoryError>;
    fn get_all_room(&self) -> Result<Vec<RoomInfo>, RepositoryError>;
    fn delete_room(&self, room_id: &str) -> Result<(), RepositoryError>;
}
