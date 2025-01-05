use axum::{
    routing::{get, post},
    Router,
};
use http::{
    header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;

use crate::{
    handlers::{
        auth::login,
        chat::chat_handler_with_upgrade,
        room::{
            create_room_handler, delete_room_handler, get_all_room_info_handler,
            get_owner_room_handler, get_specific_room_info,
        },
        users::{add_new_user, delete_user_handle, get_user_info_handle},
    },
    AppState,
};

// ルーティング処理の実装
pub fn app(app_state: AppState, origin: Vec<String>) -> Router {
    let origins: Vec<HeaderValue> = origin
        .iter()
        .map(|e| e.parse::<HeaderValue>().unwrap())
        .collect();

    Router::new()
        .route(
            "/user",
            post(add_new_user)
                .get(get_user_info_handle)
                .delete(delete_user_handle),
        )
        .route("/login", post(login))
        .route(
            "/room",
            post(create_room_handler).get(get_all_room_info_handler),
        )
        .route("/room/self", get(get_owner_room_handler))
        .route(
            "/room/:id",
            get(get_specific_room_info).delete(delete_room_handler),
        )
        // ws://localhost:8080/chat/:id
        .route("/chat/:id", get(chat_handler_with_upgrade))
        .with_state(app_state)
        .layer(
            CorsLayer::new()
                .allow_origin(origins)
                .allow_headers([
                    CONTENT_TYPE,
                    ACCESS_CONTROL_ALLOW_ORIGIN,
                    ACCESS_CONTROL_ALLOW_HEADERS,
                    ACCESS_CONTROL_ALLOW_CREDENTIALS,
                    ACCESS_CONTROL_ALLOW_METHODS,
                ])
                .allow_methods([
                    Method::POST,
                    Method::GET,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_credentials(true),
        )
}
