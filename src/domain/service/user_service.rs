use crate::domain::{
    entity::{create_user_payload::CreateUserPayload, pub_user_info::PubUserInfo, user::User},
    repository::user_repository::UserRepository,
};

use super::{
    error::ServiceError,
    util::{password_hash_service::PasswordHashService, uuid_gen::UUIDGen},
};

pub struct UserService<R, P, U>
where
    R: UserRepository,
    P: PasswordHashService,
    U: UUIDGen,
{
    repo: R,
    password_hasher: P,
    uuid: U,
}

impl<R, P, U> UserService<R, P, U>
where
    R: UserRepository,
    P: PasswordHashService,
    U: UUIDGen,
{
    pub fn new(repo: R, password_hasher: P, uuid: U) -> Self {
        Self {
            repo,
            password_hasher,
            uuid,
        }
    }

    pub async fn create_new_user(
        &self,
        new_user_payload: CreateUserPayload,
    ) -> Result<PubUserInfo, ServiceError> {
        if let Ok(_) = self.repo.get_info_mail(&new_user_payload.user_mail).await {
            return Err(ServiceError::UserAlreadyExist);
        }
        // パスワードのHASH化
        let hashed_payload = self
            .password_hasher
            .to_hash_pwd(&new_user_payload.user_pass)?;

        let user_id = self.uuid.gen();
        // ユーザーの作成
        let new_user = User {
            user_id,
            user_name: new_user_payload.user_name,
            user_mail: new_user_payload.user_mail,
            user_pass: hashed_payload,
        };

        // DB呼び出し
        let user_info = self.repo.insert(&new_user).await?;
        Ok(user_info)
    }

    pub async fn get_user_by_id(&self, user_id: String) -> Result<PubUserInfo, ServiceError> {
        let user_info = self.repo.get_user_info_id(&user_id).await?;
        Ok(user_info)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<(), ServiceError> {
        self.repo.delete(&user_id).await?;
        Ok(())
    }
}
