use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserPayload {
    #[validate(length(min = 1, max = 15))]
    pub user_name: String,
    #[validate(email)]
    pub user_mail: String,
    #[validate(length(min = 8, max = 64))]
    pub user_pass: String,
}
