use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{
        auth_handlers::login,
        room_handlers::{
            create_room_handler, delete_room_handler, get_all_room_info_handler,
            get_specific_room_info,
        },
        user_handlers::add_new_user,
    },
    AppState,
};

// ルーティング処理の実装
pub fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/user", post(add_new_user))
        .route("/login", post(login))
        .route(
            "/room",
            post(create_room_handler).get(get_all_room_info_handler),
        )
        .route(
            "/room/:id",
            get(get_specific_room_info).delete(delete_room_handler),
        )
        .with_state(app_state)
}
