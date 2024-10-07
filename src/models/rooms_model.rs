use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Sender;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateRoom {
    #[validate(length(min = 1, max = 30))]
    room_name: String,
}

impl CreateRoom {
    pub fn get_room_name(&self) -> &str {
        &self.room_name
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub room_info: RoomInfo,
    pub sender: Sender<String>,
}

impl Room {
    pub fn get_room_info(&self) -> &RoomInfo {
        &self.room_info
    }

    pub fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RoomInfo {
    pub room_id: String,
    pub room_name: String,
    pub created_by_id: String,
    pub created_by_name: String,
}

impl RoomInfo {
    pub fn get_room_id(&self) -> &str {
        &self.room_id
    }

    pub fn get_created_by_id(&self) -> &str {
        &self.created_by_id
    }
}
