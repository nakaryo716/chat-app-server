use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

use super::auth_model::Claims;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserPayload {
    #[validate(length(min = 1, max = 15))]
    pub user_name: String,
    #[validate(email)]
    pub user_mail: String,
    #[validate(length(min = 8, max = 64))]
    pub user_pass: String,
}

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

impl From<User> for PubUserInfo {
    fn from(value: User) -> Self {
        Self {
            user_id: value.get_user_id().to_string(),
            user_name: value.get_user_name().to_string(),
        }
    }
}

impl From<Claims> for PubUserInfo {
    fn from(value: Claims) -> Self {
        Self {
            user_id: value.get_user_id().to_string(),
            user_name: value.get_user_name().to_string(),
        }
    }
}
