use crate::domain::service::error::ServiceError;

pub trait PasswordHashService {
    fn to_hash_pwd(&self, password: &str) -> Result<String, ServiceError>;

    fn verify_pwd(&self, password: &str, password_hash: &str) -> Result<bool, ServiceError>;
}
