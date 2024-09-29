use axum::async_trait;

// ユーザー情報を保持したデータベースとの接続関連の実装

struct UserDb {
    pool: String,
}

#[async_trait]
trait UserDbManage {
    async fn insert_new_user();
    async fn get_user();
    async fn update_user();
    async fn delete_user();
}
