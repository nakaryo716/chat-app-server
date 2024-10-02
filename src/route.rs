use axum::{routing::post, Router};

use crate::{database::users_db::UserDb, handlers::user_handlers::add_new_user};

// ルーティング処理の実装
pub fn app(app_state: UserDb) -> Router {
    Router::new()
        .route("/user", post(add_new_user))
        .with_state(app_state)
}
