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
        // TODO: payloadのバリデーション
        if let Ok(_) = self
            .db_pool
            .get_user_info_mail(new_user_payload.user_mail.clone())
            .await
        {
            return Err(UserServiciesError::AlreadyExist);
        }
        // TODO: パスワードのHASH化
        // ユーザーの作成
        let new_user = User::new(new_user_payload);

        // detabase呼び出し
        let user_info = self
            .db_pool
            .insert_new_user(new_user)
            .await
            .map_err(|_| UserServiciesError::Unexpect)?;
        Ok(user_info)
    }

    pub async fn get_user_by_id(&self, user_id: String) -> Result<PubUserInfo, UserServiciesError> {
        let a = self
            .db_pool
            .get_user_info_id(user_id)
            .await
            .map_err(|e| match e {
                sqlx::error::Error::RowNotFound => UserServiciesError::NotFound,
                _ => UserServiciesError::Unexpect,
            })?;
        Ok(a)
    }

    pub async fn get_user_by_mail(
        &self,
        user_mail: String,
    ) -> Result<PubUserInfo, UserServiciesError> {
        let user_data = self
            .db_pool
            .get_user_info_mail(user_mail)
            .await
            .map_err(|e| match e {
                sqlx::error::Error::RowNotFound => UserServiciesError::NotFound,
                _ => UserServiciesError::Unexpect,
            })?;
        Ok(user_data)
    }

    pub async fn get_full_user_data(&self, user_mail: String) -> Result<User, UserServiciesError> {
        let user_data = self
            .db_pool
            .get_user_data(user_mail)
            .await
            .map_err(|e| match e {
                sqlx::error::Error::RowNotFound => UserServiciesError::NotFound,
                _ => UserServiciesError::Unexpect,
            })?;
        Ok(user_data)
    }
}

#[derive(Debug)]
pub enum UserServiciesError {
    NotFound,
    AlreadyExist,
    Unexpect,
}
