use argon2::{password_hash::Salt, Argon2, PasswordHasher};

use crate::{
    database::users_db::{UserDataViewer, UserDbManage},
    models::user_model::{CreateUserPayload, PubUserInfo, User},
};

pub struct UserServices<'a, T>
where
    T: UserDbManage<User, String> + UserDataViewer<String, String>,
{
    db_pool: &'a T,
}

impl<'a, T> UserServices<'a, T>
where
    T: UserDbManage<User, String, UserInfo = PubUserInfo, Error = sqlx::error::Error>,
    T: UserDataViewer<
        String,
        String,
        FullUserData = User,
        UserInfo = PubUserInfo,
        Error = sqlx::error::Error,
    >,
{
    pub fn new(db_pool: &'a T) -> Self {
        Self { db_pool }
    }

    pub async fn create_new_user(
        &self,
        new_user_payload: CreateUserPayload,
    ) -> Result<PubUserInfo, UserServiciesError> {
        if let Ok(_) = self
            .db_pool
            .get_user_info_mail(new_user_payload.user_mail.clone())
            .await
        {
            return Err(UserServiciesError::AlreadyExist);
        }
        // パスワードのHASH化
        let hashed_user_payload = UserServices::<T>::to_hash_pwd(new_user_payload)?;
        // ユーザーの作成
        let new_user = User::new(hashed_user_payload);

        // detabase呼び出し
        let user_info = self
            .db_pool
            .insert_new_user(new_user)
            .await
            .map_err(|_| UserServiciesError::Unexpect)?;
        Ok(user_info)
    }

    pub async fn get_user_by_id(&self, user_id: String) -> Result<PubUserInfo, UserServiciesError> {
        let user_info = self
        .db_pool
        .get_user_info_id(user_id)
        .await
        .map_err(|e| match e {
                sqlx::error::Error::RowNotFound => UserServiciesError::NotFound,
                _ => UserServiciesError::Unexpect,
            })?;
        Ok(user_info)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<(), UserServiciesError> {
        self.db_pool
        .delete_user(user_id.to_string())
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => UserServiciesError::NotFound,
            _ => UserServiciesError::Unexpect,
        })?;
        Ok(())
    }

    fn to_hash_pwd(payload: CreateUserPayload) -> Result<CreateUserPayload, UserServiciesError> {
        let salt = dotenvy::var("SALT").map_err(|_| UserServiciesError::ToHash)?;
        let salt = Salt::from_b64(&salt).map_err(|_| UserServiciesError::ToHash)?;
    
        let row_pwd = payload.user_pass;
        let hashed_pwd = Argon2::default()
            .hash_password(row_pwd.as_bytes(), salt)
            .map_err(|_| UserServiciesError::ToHash)?
            .to_string();
    
        let hashed_payload = CreateUserPayload {
            user_pass: hashed_pwd,
            ..payload
        };
        Ok(hashed_payload)
    }
}

#[derive(Debug)]
pub enum UserServiciesError {
    NotFound,
    AlreadyExist,
    Unexpect,
    ToHash,
}
