use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    #[validate(email)]
    pub client_mail: String,
    #[validate(length(min = 8, max = 64))]
    pub client_pass: String,
}
