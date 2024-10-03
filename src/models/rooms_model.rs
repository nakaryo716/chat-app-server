use serde::Deserialize;
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone)]
pub struct RoomInfo {
    pub room_id: String,
    pub room_name: String,
    pub created_by: String,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub room_info: RoomInfo,
    pub sender: Sender<String>,
}

impl RoomInfo {
    pub fn get_room_id(&self) -> &str {
        &self.room_id
    }

    pub fn get_room_name(&self) -> &str {
        &self.room_name
    }

    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
}

impl Room {
    pub fn get_room_info(&self) -> &RoomInfo {
        &self.room_info
    }

    pub fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoom {
    room_name: String,
}

impl CreateRoom {
    pub fn get_room_name(&self) -> &str {
        &self.room_name
    }
}
