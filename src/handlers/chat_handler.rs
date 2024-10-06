use axum::extract::{Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::warn;

use crate::database::rooms_db::RoomDb;
use crate::models::auth_model::Claims;
use crate::models::user_model::PubUserInfo;
use crate::services::chat_ws::ChatServices;
use crate::services::rooms::RoomServices;

pub async fn chat_handler_with_upgrade(
    claims: Claims,
    Path(room_id): Path<String>,
    State(room_db): State<RoomDb>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let room_sender = match RoomServices::new(&room_db).get_sender(&room_id) {
        Ok(sender) => sender,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };
    let user_info = PubUserInfo::from(claims);

    ws.on_failed_upgrade(|e| warn!("websocket upgrade error {}", e))
        .on_upgrade(move |socket| {
            let chat_services = ChatServices::new(socket, room_sender, user_info);
            chat_services.ws_task()
        })
}
