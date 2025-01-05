use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccessToken(pub String);
