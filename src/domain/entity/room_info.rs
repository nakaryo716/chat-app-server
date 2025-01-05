use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfo {
    pub room_id: String,
    pub room_name: String,
    pub created_by_id: String,
    pub created_by_name: String,
    pub created_time: DateTime<Utc>,
}
