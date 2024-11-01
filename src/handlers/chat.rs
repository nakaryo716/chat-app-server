use axum::extract::{Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tracing::warn;

use crate::auth::Claims;
use crate::chat::services::ChatServices;
use crate::room::database::RoomDb;
use crate::room::services::RoomServices;
use crate::users::PubUserInfo;

pub async fn chat_handler_with_upgrade(
    claims: Claims,
    Path(room_id): Path<String>,
    State(room_db): State<RoomDb>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let room_sender = match RoomServices::new(&room_db).get_sender(&room_id) {
        Ok(sender) => sender,
        Err(_) => {
            let body = Json(json!({
                "error": "Room not found",
            }));
            return (StatusCode::NOT_FOUND, body).into_response();
        }
    };
    let user_info = PubUserInfo::from(claims);

    ws.on_failed_upgrade(|e| warn!("websocket upgrade error {}", e))
        .on_upgrade(move |socket| {
            let chat_services = ChatServices::new(socket, room_sender, user_info);
            chat_services.ws_task()
        })
}
