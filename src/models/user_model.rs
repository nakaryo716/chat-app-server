use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct User {
    user_id: String,
    user_name: String,
    user_mail: String,
    user_pass: String,
}

impl User {
    pub fn new(payload: CreateUserPayload) -> Self {
        let user_id = Uuid::new_v4().to_string();
        Self {
            user_id,
            user_name: payload.user_name,
            user_mail: payload.user_mail,
            user_pass: payload.user_pass,
        }
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }
    pub fn get_user_mail(&self) -> &str {
        &self.user_mail
    }
    pub fn get_user_pass(&self) -> &str {
        &self.user_pass
    }
}
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PubUserInfo {
    user_id: String,
    user_name: String,
}

impl PubUserInfo {
    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_user_name(&self) -> &str {
        &self.user_name
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserPayload {
    pub user_name: String,
    pub user_mail: String,
    pub user_pass: String,
}
