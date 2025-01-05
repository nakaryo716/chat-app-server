use std::{future::Future, pin::Pin};

use sqlx::PgPool;

use crate::domain::{
    entity::{pub_user_info::PubUserInfo, user::User},
    repository::user_repository::UserRepository,
};

pub struct UserRepositoryImpl<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepositoryImpl<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
}

impl<'r> UserRepository for UserRepositoryImpl<'r> {
    fn insert<'a>(
        &'a self,
        user: &'a User,
    ) -> Pin<Box<dyn Future<Output = Result<PubUserInfo, sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            let data: PubUserInfo = sqlx::query_as(
                r#"
                INSERT INTO user_data
                (user_id, user_name, user_mail, user_pass)
                VALUES ($1, $2, $3, $4)
                RETURNING user_id, user_name
                "#,
            )
            .bind(&user.user_id)
            .bind(&user.user_name)
            .bind(&user.user_mail)
            .bind(&user.user_pass)
            .fetch_one(self.pool)
            .await?;
            Ok(data)
        })
    }

    fn get_user_data<'a>(
        &'a self,
        user_mail: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<User, sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            let data: User = sqlx::query_as(
                r#"
                SELECT user_id, user_name, user_mail, user_pass
                FROM user_data
                WHERE user_mail = $1
                "#,
            )
            .bind(user_mail)
            .fetch_one(self.pool)
            .await?;
            Ok(data)
        })
    }

    fn get_user_info_id<'a>(
        &'a self,
        user_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<PubUserInfo, sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            let data: PubUserInfo = sqlx::query_as(
                r#"
                SELECT user_id, user_name FROM user_data
                WHERE user_id = $1
                "#,
            )
            .bind(user_id)
            .fetch_one(self.pool)
            .await?;
            Ok(data)
        })
    }

    fn get_info_mail<'a>(
        &'a self,
        user_mail: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<PubUserInfo, sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            let data: PubUserInfo = sqlx::query_as(
                r#"
                SELECT user_id, user_name FROM user_data
                WHERE user_mail = $1
                "#,
            )
            .bind(user_mail)
            .fetch_one(self.pool)
            .await?;
            Ok(data)
        })
    }

    fn delete<'a>(
        &'a self,
        user_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            let r = sqlx::query(
                r#"
                DELETE FROM user_data
                WHERE user_id = $1
                "#,
            )
            .bind(user_id)
            .execute(self.pool)
            .await?;

            if r.rows_affected() >= 1 {
                Ok(())
            } else {
                Err(sqlx::Error::RowNotFound)
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entity::create_user_payload::CreateUserPayload;

    use super::*;
    use rand::random;

    async fn set_up_db() -> PgPool {
        let url = dotenvy::var("DATABASE_URL").unwrap();
        let pool = PgPool::connect(&url).await.unwrap();
        pool
    }

    fn gen_random_user() -> User {
        let random_num = random::<f64>();
        let payload = CreateUserPayload {
            user_name: format!("test-user-name{}", random_num),
            user_mail: format!("test-user-mail{}", random_num),
            user_pass: format!("test-user-pass{}", random_num),
        };

        User {
            user_id: format!("user_id_{}", random_num),
            user_name: payload.user_name,
            user_mail: payload.user_mail,
            user_pass: payload.user_pass,
        }
    }

    #[tokio::test]
    async fn test_insert_new_user() {
        let pool = set_up_db().await;
        let new_user = gen_random_user();

        let repo = UserRepositoryImpl::new(&pool);
        // テスト対象
        let res_user_info = repo.insert(&new_user).await.unwrap();

        assert_eq!(res_user_info.user_id, new_user.user_id);
        assert_eq!(res_user_info.user_name, new_user.user_name);

        // 削除
        repo.delete(&res_user_info.user_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_new_user() {
        let pool = set_up_db().await;
        let new_user = gen_random_user();
        let repo = UserRepositoryImpl::new(&pool);
        // 最初にユーザー登録
        let res_user_info = repo.insert(&new_user).await.unwrap();

        // テスト対象
        repo.delete(&res_user_info.user_id).await.unwrap();

        // 再度クエリし、エラーが返ることを確認する
        // エラーはRowNotFoundになる
        let query_result = repo.get_user_data(&new_user.user_mail).await;
        match query_result {
            Ok(_) => {
                panic!("Expect Row Not Found");
            }
            Err(e) => {
                // sqlx::error::ErrorがPartialEqを実装していない
                // thisErrorが実装されているため、そこで記述されているFmtを比較する
                let err_txt = e.to_string();
                let sqlx_error_msg =
                    "no rows returned by a query that expected to return at least one row";
                assert_eq!(err_txt, sqlx_error_msg);
            }
        }
    }

    #[tokio::test]
    async fn test_get_user_data() {
        let pool = set_up_db().await;
        let new_user = gen_random_user();
        let repo = UserRepositoryImpl::new(&pool);
        // 最初に登録
        let res_user_info = repo.insert(&new_user).await.unwrap();

        // テスト対象
        let full_data_result = repo.get_user_data(&new_user.user_mail).await.unwrap();
        assert_eq!(full_data_result, new_user);

        // 削除
        repo.delete(&res_user_info.user_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_user_info_id() {
        let pool = set_up_db().await;
        let new_user = gen_random_user();
        let repo = UserRepositoryImpl::new(&pool);
        // 最初に登録
        repo.insert(&new_user).await.unwrap();

        // テスト対象
        let user_info_result = repo.get_user_info_id(&new_user.user_id).await.unwrap();
        assert_eq!(user_info_result.user_id, new_user.user_id);
        assert_eq!(user_info_result.user_name, new_user.user_name);

        // 削除
        repo.delete(&user_info_result.user_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_user_info_mail() {
        let pool = set_up_db().await;
        let new_user = gen_random_user();
        let repo = UserRepositoryImpl::new(&pool);
        // 最初に登録
        repo.insert(&new_user).await.unwrap();

        // テスト対象
        let user_info_result = repo.get_info_mail(&new_user.user_mail).await.unwrap();
        assert_eq!(user_info_result.user_id, new_user.user_id);
        assert_eq!(user_info_result.user_name, new_user.user_name);

        // 削除
        repo.delete(&user_info_result.user_id).await.unwrap();
    }
}
