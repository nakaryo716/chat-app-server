use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PubUserInfo {
    pub user_id: String,
    pub user_name: String,
}
