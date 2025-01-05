use std::{future::Future, pin::Pin};

use crate::domain::entity::{pub_user_info::PubUserInfo, user::User};

pub trait UserRepository {
    fn insert<'a>(
        &'a self,
        user: &'a User,
    ) -> Pin<Box<dyn Future<Output = Result<PubUserInfo, sqlx::Error>> + Send + 'a>>;

    fn get_user_data<'a>(
        &'a self,
        user_mail: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<User, sqlx::Error>> + Send + 'a>>;

    fn get_user_info_id<'a>(
        &'a self,
        user_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<PubUserInfo, sqlx::Error>> + Send + 'a>>;

    fn get_info_mail<'a>(
        &'a self,
        user_mail: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<PubUserInfo, sqlx::Error>> + Send + 'a>>;

    fn delete<'a>(
        &'a self,
        user_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), sqlx::Error>> + Send + 'a>>;
}
