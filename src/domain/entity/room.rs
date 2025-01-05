use tokio::sync::broadcast::Sender;

use super::room_info::RoomInfo;

#[derive(Debug, Clone)]
pub struct Room {
    pub room_info: RoomInfo,
    pub sender: Sender<String>,
}
