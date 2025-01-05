use crate::domain::{entity::claims::Claims, service::error::ServiceError};

pub trait TokenService {
    fn encode(&self, claims: &Claims) -> Result<String, ServiceError>;
    fn decode(&self, token: &str) -> Result<Claims, ServiceError>;
}
