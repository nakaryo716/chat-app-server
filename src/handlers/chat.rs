use axum::extract::{Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tracing::warn;

use crate::domain::entity::claims::Claims;
use crate::domain::entity::pub_user_info::PubUserInfo;
use crate::domain::service::chat_service::ChatServices;
use crate::domain::service::room_service::RoomServices;
use crate::infrastructure::repository::room_repository_impl::RoomRepositoryImpl;
use crate::RoomDb;

pub async fn chat_handler_with_upgrade(
    claims: Claims,
    Path(room_id): Path<String>,
    State(repo): State<RoomDb>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let service = RoomServices::new(RoomRepositoryImpl::new(repo));

    let room_sender = match service.get_sender(&room_id) {
        Ok(sender) => sender,
        Err(_) => {
            let body = Json(json!({
                "error": "Room not found",
            }));
            return (StatusCode::NOT_FOUND, body).into_response();
        }
    };
    let user_info = PubUserInfo {
        user_id: claims.user_id,
        user_name: claims.user_name,
    };

    ws.on_failed_upgrade(|e| warn!("websocket upgrade error {}", e))
        .on_upgrade(move |socket| {
            let chat_services = ChatServices::new(socket, room_sender, user_info);
            chat_services.ws_task()
        })
}
