use std::{
    collections::HashMap,
    sync::{RwLockReadGuard, RwLockWriteGuard},
};

use chrono::Utc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{
    domain::{
        entity::{pub_user_info::PubUserInfo, room::Room, room_info::RoomInfo},
        repository::{error::RepositoryError, room_repository::RoomRepository},
    },
    RoomDb,
};

pub struct RoomRepositoryImpl {
    db: RoomDb,
}

impl RoomRepositoryImpl {
    pub fn new(db: RoomDb) -> Self {
        Self { db }
    }
}

impl RoomRepository for RoomRepositoryImpl {
    fn open_new_room(
        &self,
        room_name: &str,
        user_info: &PubUserInfo,
    ) -> Result<RoomInfo, RepositoryError> {
        let room = init_room(room_name, &user_info.user_id, &user_info.user_name);

        {
            let mut guard = get_write_lock(&self).map_err(|e| e)?;
            guard.insert(room.room_info.room_id.clone(), room.clone());
        }
        Ok(room.room_info.to_owned())
    }

    fn listen_room(&self, room_id: &str) -> Result<Room, RepositoryError> {
        let room = get_read_lock(&self).and_then(|guard| {
            guard
                .get(room_id)
                .map(|r| r.to_owned())
                .ok_or_else(|| RepositoryError::NotFound)
        })?;
        Ok(room)
    }

    fn get_owner_rooms(&self, owner_id: &str) -> Result<Vec<RoomInfo>, RepositoryError> {
        let lock = get_read_lock(&self).map_err(|e| e)?;

        let owner_rooms = lock
            .iter()
            .filter(|(_, room)| room.room_info.created_by_id == owner_id)
            .map(|(_, b)| b.room_info.to_owned())
            .collect();
        Ok(owner_rooms)
    }

    fn get_all_room(&self) -> Result<Vec<RoomInfo>, RepositoryError> {
        let lock = get_read_lock(&self).map_err(|e| e)?;
        let rooms: Vec<RoomInfo> = lock
            .iter()
            .map(|(_, room)| room.room_info.to_owned())
            .collect();
        Ok(rooms)
    }

    fn delete_room(&self, room_id: &str) -> Result<(), RepositoryError> {
        let _ = get_write_lock(&self).and_then(|mut guard| {
            guard
                .remove(room_id)
                .ok_or_else(|| RepositoryError::NotFound)
        });
        Ok(())
    }
}

// ユニークIDを割り振る
// チャンネルの作成を行う
fn init_room(room_name: &str, created_by_id: &str, user_name: &str) -> Room {
    let (sender, _) = broadcast::channel(128);

    Room {
        room_info: RoomInfo {
            room_id: Uuid::new_v4().to_string(),
            room_name: room_name.to_owned(),
            created_by_id: created_by_id.to_owned(),
            created_by_name: user_name.to_owned(),
            created_time: Utc::now(),
        },
        sender,
    }
}

fn get_write_lock(
    repo: &RoomRepositoryImpl,
) -> Result<RwLockWriteGuard<HashMap<String, Room>>, RepositoryError> {
    let lock = repo.db.pool.write().map_err(|_| RepositoryError::DbError)?;
    Ok(lock)
}

fn get_read_lock(
    repo: &RoomRepositoryImpl,
) -> Result<RwLockReadGuard<HashMap<String, Room>>, RepositoryError> {
    let lock = repo.db.pool.read().map_err(|_| RepositoryError::DbError)?;
    Ok(lock)
}
