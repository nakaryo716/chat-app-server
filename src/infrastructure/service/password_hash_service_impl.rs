use std::sync::LazyLock;

use argon2::{
    password_hash::{self, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use rand_core::OsRng;

use crate::domain::service::{
    error::ServiceError, util::password_hash_service::PasswordHashService,
};

static ARGON2: LazyLock<Argon2> = LazyLock::new(|| {
    let argon = Argon2::new(
        // Algorithm type
        Algorithm::Argon2id,
        // Version
        Version::V0x13,
        // Parameters(cost)
        Params::new(19_456u32, 2u32, 1u32, Some(32usize))
            .expect("Failed to initialize Argon2 parameters"),
    );
    argon
});

pub struct PasswordHashServiceImpl;

impl PasswordHashService for PasswordHashServiceImpl {
    fn to_hash_pwd(
        &self,
        password: &str,
    ) -> Result<String, crate::domain::service::error::ServiceError> {
        let argon2 = &ARGON2;
        let salt = SaltString::generate(OsRng);
        let hash_password = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| ServiceError::ToHash)?
            .to_string();
        Ok(hash_password)
    }

    fn verify_pwd(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, crate::domain::service::error::ServiceError> {
        let argon2 = &ARGON2;
        let hash_password = PasswordHash::new(&password_hash).map_err(|_| ServiceError::ToHash)?;
        match argon2.verify_password(password.as_bytes(), &hash_password) {
            Ok(_) => Ok(true),
            Err(e) => match e {
                password_hash::Error::Password => Ok(false),
                _ => Err(ServiceError::ToHash),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use super::PasswordHashServiceImpl;

    #[test]
    fn test_same_password() {
        let password = "#password!";
        let hash_password = PasswordHashServiceImpl.to_hash_pwd(password).unwrap();
        let verify_result = PasswordHashServiceImpl
            .verify_pwd(password, &hash_password)
            .unwrap();
        assert!(verify_result);
    }

    #[test]
    fn test_wrong_password() {
        let password = "#password!";
        let wrong_password = "!wrong_password?";

        let hash_password = PasswordHashServiceImpl.to_hash_pwd(password).unwrap();
        let verify_result = PasswordHashServiceImpl
            .verify_pwd(&wrong_password, &hash_password)
            .unwrap();
        assert!(!verify_result);
    }

    // レインボーテーブル攻撃に対して、SaltStringが機能しているかのテスト
    #[test]
    fn test_different_hash() {
        let password = "#password!";

        let hash_password1 = PasswordHashServiceImpl.to_hash_pwd(password).unwrap();
        let hash_password2 = PasswordHashServiceImpl.to_hash_pwd(password).unwrap();
        assert_ne!(hash_password1, hash_password2);
    }
}
