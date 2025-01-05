use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    user_id: String,
    user_name: String,
    text: String,
    time: DateTime<Utc>,
}

impl Chat {
    pub fn from_str(user_id: &str, user_name: &str, text: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            user_name: user_name.to_string(),
            text: text.to_string(),
            time: Utc::now(),
        }
    }
}
