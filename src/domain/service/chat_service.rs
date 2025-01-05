use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast::Sender;
use tracing::warn;

use crate::domain::entity::{chat::Chat, pub_user_info::PubUserInfo};

pub struct ChatServices {
    socket: WebSocket,
    room_sender: Sender<String>,
    user_info: PubUserInfo,
}

impl ChatServices {
    pub fn new(socket: WebSocket, room_sender: Sender<String>, user_info: PubUserInfo) -> Self {
        Self {
            socket,
            room_sender,
            user_info,
        }
    }

    pub async fn ws_task(self) {
        let (mut ws_sender, mut ws_receiver) = self.socket.split();

        let room_sender = self.room_sender.clone();
        let mut receive_task = tokio::task::spawn(async move {
            while let Some(Ok(Message::Text(sended_text))) = ws_receiver.next().await {
                if sended_text.len() == 0 {
                    continue;
                }

                let chat_msg = Chat::from_str(
                    &self.user_info.user_id,
                    &self.user_info.user_name,
                    &sended_text,
                );

                let Ok(serialized_chat_msg) = serde_json::to_string(&chat_msg) else {
                    if let Err(e) = room_sender.send("failed to serialize chat message".to_string())
                    {
                        warn!("websocket receive task error: {:?}", e);
                        break;
                    }
                    break;
                };

                if let Err(e) = room_sender.send(serialized_chat_msg) {
                    warn!("websocket receive task error: {:?}", e);
                    break;
                }
            }
        });

        let mut room_receiver = self.room_sender.subscribe();
        let mut send_task = tokio::task::spawn(async move {
            while let Ok(chat_msg) = room_receiver.recv().await {
                if let Err(e) = ws_sender.send(Message::Text(chat_msg)).await {
                    warn!("websocket send task error: {:?}", e);
                    break;
                }
            }
        });

        tokio::select! {
            _ = &mut send_task => receive_task.abort(),
            _ = &mut receive_task => send_task.abort(),
        };
    }
}
