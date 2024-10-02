use serde::Deserialize;
use tokio::sync::broadcast::Sender;

use crate::{database::rooms_db::{init_room, RoomDb, RoomError}, models::{rooms_model::RoomInfo, user_model::PubUserInfo}};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoom {
    room_name: String,
}

pub struct Error{}

pub fn create_room(db: &RoomDb, payload: CreateRoom, user_info: PubUserInfo) -> Result<RoomInfo, RoomError>{
    // TODO: ルーム名のバリデーションを行う
    let room = init_room(&payload.room_name, user_info.get_user_id());
    db.open_new_room(room.clone()).map_err(|e| e)?;
    Ok(room.get_room_info().to_owned())
}

pub fn get_target_room_info(db: &RoomDb, room_id: &str) -> Result<RoomInfo, RoomError> {
    let room =db.listen_room(room_id).map_err(|e| e)?;
    Ok(room.get_room_info().to_owned())
}

pub fn get_sender(db: &RoomDb, room_id: &str) -> Result<Sender<String>, RoomError> {
    let room = db.listen_room(room_id).map_err(|e| e)?;
    Ok(room.get_sender())
}

pub fn delete_owner_room(db: &RoomDb, room_id: &str, user_info: PubUserInfo) -> Result<(), Error> {
    let room = db.listen_room(room_id).map_err(|_| Error{})?;
    let room_owner = room.get_room_info().get_created_by();

    if room_owner == user_info.get_user_id() {
        db.delete_room(room_id).map_err(|_| Error{})
    } else {
        Err(Error{})
    }
}
