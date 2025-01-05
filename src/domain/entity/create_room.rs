use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoom {
    #[validate(length(min = 1, max = 30))]
    pub room_name: String,
}
