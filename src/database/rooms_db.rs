use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use tokio::sync::broadcast;
use uuid::Uuid;

use crate::models::rooms_model::{Room, RoomInfo};

pub struct RoomDb {
    pool: Arc<Mutex<HashMap<String, Room>>>,
}

pub enum RoomError {
    DbError,
    IdNotFound,
}

impl RoomDb {
    pub fn open_new_room(&self, room: Room) -> Result<(), RoomError> {
        let mut gurad = get_lock(&self).map_err(|e| e)?;
        gurad.insert(room.get_room_info().get_room_id().to_string(), room);

        Ok(())
    }

    pub fn listen_room(&self, room_id: &str) -> Result<Room, RoomError> {
        let room = get_lock(&self).and_then(|guard| {
            guard
                .get(room_id)
                .map(|e| e.to_owned())
                .ok_or_else(|| RoomError::IdNotFound)
        })?;
        Ok(room)
    }

    pub fn delete_room(&self, room_id: &str) -> Result<(), RoomError>{
        let _ = get_lock(&self).and_then(|mut gurad|{
            gurad.remove(room_id).ok_or_else(|| RoomError::IdNotFound)
        });
        Ok(())
    }
}

pub fn init_room(room_name: &str, created_by_id: &str) -> Room {
    let (sender, _) = broadcast::channel(128);

    Room {
        room_info: RoomInfo {
            room_id: Uuid::new_v4().to_string(),
            room_name: room_name.to_owned(),
            created_by: created_by_id.to_owned(),
        },
        sender,
    }
}

fn get_lock(db: &RoomDb) -> Result<MutexGuard<HashMap<String, Room>>, RoomError> {
    let lock = db.pool.lock().map_err(|_| RoomError::DbError)?;
    Ok(lock)
}
