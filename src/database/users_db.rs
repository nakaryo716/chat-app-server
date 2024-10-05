use crate::{
    models::user_model::{PubUserInfo, User},
    AppState,
};
use axum::{async_trait, extract::FromRef};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct UserDb {
    pool: PgPool,
}

impl UserDb {
    pub async fn connect(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = PgPool::connect(database_url).await.map_err(|e| e)?;
        Ok(Self { pool })
    }
}

#[async_trait]
pub trait UserDbManage<User, Id> {
    type UserInfo;
    type Error;

    async fn insert_new_user(&self, new_user: User) -> Result<Self::UserInfo, Self::Error>;
    async fn delete_user(&self, user_id: Id) -> Result<(), Self::Error>;
}

#[async_trait]
impl UserDbManage<User, String> for UserDb {
    type UserInfo = PubUserInfo;
    type Error = sqlx::error::Error;

    async fn insert_new_user(&self, user: User) -> Result<Self::UserInfo, Self::Error> {
        let data: PubUserInfo = sqlx::query_as(
            r#"
            INSERT INTO user_data
            (user_id, user_name, user_mail, user_pass)
            VALUES ($1, $2, $3, $4)
            RETURNING user_id, user_name
            "#,
        )
        .bind(user.get_user_id())
        .bind(user.get_user_name())
        .bind(user.get_user_mail())
        .bind(user.get_user_pass())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e)?;

        Ok(data)
    }

    async fn delete_user(&self, user_id: String) -> Result<(), Self::Error> {
        sqlx::query(
            r#"
            DELETE FROM user_data
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| e)?;
        Ok(())
    }
}

#[async_trait]
pub trait UserDataViewer<Id, Mail> {
    type FullUserData;
    type UserInfo;
    type PassWord;
    type Error;
    async fn get_user_data(&self, user_mail: Mail) -> Result<Self::FullUserData, Self::Error>;
    async fn get_user_info_id(&self, user_id: Id) -> Result<Self::UserInfo, Self::Error>;
    async fn get_user_info_mail(&self, user_mail: Mail) -> Result<Self::UserInfo, Self::Error>;
}

#[async_trait]
impl UserDataViewer<String, String> for UserDb {
    type FullUserData = User;
    type UserInfo = PubUserInfo;
    type PassWord = String;
    type Error = sqlx::error::Error;

    async fn get_user_data(&self, user_mail: String) -> Result<Self::FullUserData, Self::Error> {
        let data: User = sqlx::query_as(
            r#"
            SELECT user_id, user_name, user_mail, user_pass
            FROM user_data
            WHERE user_mail = $1
            "#,
        )
        .bind(user_mail)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e)?;

        Ok(data)
    }

    async fn get_user_info_id(&self, user_id: String) -> Result<Self::UserInfo, Self::Error> {
        let data: PubUserInfo = sqlx::query_as(
            r#"
            SELECT user_id, user_name FROM user_data
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e)?;
        Ok(data)
    }

    async fn get_user_info_mail(&self, user_mail: String) -> Result<Self::UserInfo, Self::Error> {
        let data: PubUserInfo = sqlx::query_as(
            r#"
            SELECT user_id, user_name FROM user_data
            WHERE user_mail = $1
            "#,
        )
        .bind(user_mail)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e)?;
        Ok(data)
    }
}

impl FromRef<AppState> for UserDb {
    fn from_ref(input: &AppState) -> Self {
        input.user_db.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::user_model::CreateUserPayload;
    use rand::random;

    async fn set_up_db() -> UserDb {
        let url = dotenvy::var("DATABASE_URL").unwrap();
        UserDb::connect(&url).await.unwrap()
    }

    fn gen_random_user() -> User {
        let random_num = random::<f64>();
        let payload = CreateUserPayload {
            user_name: format!("test-user-name{}", random_num),
            user_mail: format!("test-user-mail{}", random_num),
            user_pass: format!("test-user-pass{}", random_num),
        };
        User::new(payload)
    }

    #[tokio::test]
    async fn test_insert_new_user() {
        let db = set_up_db().await;
        let new_user = gen_random_user();

        // テスト対象
        let res_user_info = db.insert_new_user(new_user.clone()).await.unwrap();

        assert_eq!(res_user_info.get_user_id(), new_user.get_user_id());
        assert_eq!(res_user_info.get_user_name(), new_user.get_user_name());

        // 削除
        db.delete_user(res_user_info.get_user_id().to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_delete_new_user() {
        let db = set_up_db().await;
        let new_user = gen_random_user();
        // 最初にユーザー登録
        let res_user_info = db.insert_new_user(new_user.clone()).await.unwrap();

        // テスト対象
        db.delete_user(res_user_info.get_user_id().to_string())
            .await
            .unwrap();

        // 再度クエリし、エラーが返ることを確認する
        // エラーはRowNotFoundになる
        let query_result = db.get_user_data(new_user.get_user_mail().to_string()).await;
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

        // 削除
        db.delete_user(res_user_info.get_user_id().to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_user_data() {
        let db = set_up_db().await;
        let new_user = gen_random_user();
        // 最初に登録
        let res_user_info = db.insert_new_user(new_user.clone()).await.unwrap();

        // テスト対象
        let full_data_result = db
            .get_user_data(new_user.get_user_mail().to_string())
            .await
            .unwrap();
        assert_eq!(full_data_result, new_user);

        // 削除
        db.delete_user(res_user_info.get_user_id().to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_user_info_id() {
        let db = set_up_db().await;
        let new_user = gen_random_user();
        // 最初に登録
        db.insert_new_user(new_user.clone()).await.unwrap();

        // テスト対象
        let user_info_result = db
            .get_user_info_id(new_user.get_user_id().to_string())
            .await
            .unwrap();
        assert_eq!(user_info_result.get_user_id(), new_user.get_user_id());
        assert_eq!(user_info_result.get_user_name(), new_user.get_user_name());

        // 削除
        db.delete_user(user_info_result.get_user_id().to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_user_info_mail() {
        let db = set_up_db().await;
        let new_user = gen_random_user();
        // 最初に登録
        db.insert_new_user(new_user.clone()).await.unwrap();

        // テスト対象
        let user_info_result = db
            .get_user_info_mail(new_user.get_user_mail().to_string())
            .await
            .unwrap();
        assert_eq!(user_info_result.get_user_id(), new_user.get_user_id());
        assert_eq!(user_info_result.get_user_name(), new_user.get_user_name());

        // 削除
        db.delete_user(user_info_result.get_user_id().to_string())
            .await
            .unwrap();
    }
}
