use std::{
    fs::{exists, File},
    io::{Read, Write},
    sync::LazyLock,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand_core::{OsRng, RngCore};

use crate::domain::{
    entity::claims::Claims,
    service::{error::ServiceError, util::token_service::TokenService},
};

static KEY_PATH: &str = "./jwt_key.txt";

static KEY: LazyLock<String> = LazyLock::new(|| {
    if !exists(KEY_PATH).expect("cannot check key file existence") {
        let key_string = generate_key();
        create_key_file(&key_string).expect("cannot create key file");
        key_string
    } else {
        read_key_from_file()
    }
});

fn read_key_from_file() -> String {
    let mut file = File::open(KEY_PATH).expect("cannot open key file");
    let mut key = String::new();
    file.read_to_string(&mut key).expect("cannot read key file");
    key
}

fn generate_key() -> String {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    BASE64_STANDARD.encode(&key)
}

fn create_key_file(key: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(KEY_PATH)?;
    file.write_all(key.as_bytes())?;
    Ok(())
}

pub struct TokenServiceImpl;

impl TokenService for TokenServiceImpl {
    fn encode(
        &self,
        claims: &crate::domain::entity::claims::Claims,
    ) -> Result<String, crate::domain::service::error::ServiceError> {
        let header = Header::new(Algorithm::HS512);
        let key = &KEY;
        let token = encode(&header, &claims, &EncodingKey::from_secret(key.as_bytes()))
            .map_err(|_| ServiceError::TokenCreation)?;
        Ok(token)
    }

    fn decode(
        &self,
        token: &str,
    ) -> Result<crate::domain::entity::claims::Claims, crate::domain::service::error::ServiceError>
    {
        let key = &KEY;
        let token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(key.as_bytes()),
            &Validation::new(Algorithm::HS512),
        )
        .map_err(|_| ServiceError::TokenVerify)?;
        Ok(token.claims)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_read_key_from_file() {
        let key_content = "test_key_content";
        let mut file = File::create(KEY_PATH).expect("cannot create test key file");
        file.write_all(key_content.as_bytes())
            .expect("cannot write to test key file");

        let key = read_key_from_file();
        assert_eq!(key, key_content);

        std::fs::remove_file(KEY_PATH).expect("cannot delete test key file");
    }

    #[test]
    fn test_generate_key() {
        let key = generate_key();
        let decoded_key = BASE64_STANDARD
            .decode(key.as_bytes())
            .expect("cannot decode key");
        assert_eq!(decoded_key.len(), 32);
    }

    #[test]
    fn test_create_key_file() {
        let key_content = "test_key_content";
        create_key_file(key_content).expect("cannot create key file");

        let mut file = File::open(KEY_PATH).expect("cannot open test key file");
        let mut key = String::new();
        file.read_to_string(&mut key)
            .expect("cannot read test key file");
        assert_eq!(key, key_content);

        std::fs::remove_file(KEY_PATH).expect("cannot delete test key file");
    }

    #[test]
    fn test_token_service_create() {
        let service = TokenServiceImpl;
        let claims = Claims {
            user_id: "user_id".to_string(),
            user_name: "user_name".to_string(),
            exp: 10000000000,
        };

        let token = service.encode(&claims).expect("failed to create token");
        assert!(!token.is_empty());
    }

    #[test]
    fn test_token_service_verify() {
        let service = TokenServiceImpl;
        let claims = Claims {
            user_id: "user_id".to_string(),
            user_name: "user_name".to_string(),
            exp: 10000000000,
        };

        let token = service.encode(&claims).expect("failed to create token");
        let decoded = service.decode(&token).expect("failed to verify token");
        assert_eq!(claims.user_id, decoded.user_id);
        assert_eq!(claims.user_name, decoded.user_name);
    }

    #[test]
    fn test_verify_invalid_key() {
        let service = TokenServiceImpl;
        let claims = Claims {
            user_id: "user_id".to_string(),
            user_name: "user_name".to_string(),
            exp: 10000000000,
        };

        let token = service.encode(&claims).expect("failed to create token");

        // 別のキーで検証
        let invalid_key = BASE64_STANDARD.encode(&[1u8; 32]);
        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(invalid_key.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );

        assert!(result.is_err());
    }
}
