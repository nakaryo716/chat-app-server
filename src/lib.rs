use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::extract::FromRef;
use domain::entity::room::Room;
use sqlx::PgPool;

pub mod domain;
pub mod handlers;
pub mod infrastructure;
pub mod route;

mod util;

#[derive(Debug, Clone)]
pub struct AppState {
    room_db: RoomDb,
    user_db: UserDb,
}

impl AppState {
    pub fn new(room_db: RoomDb, user_db: UserDb) -> Self {
        Self { room_db, user_db }
    }
}

#[derive(Debug, Clone)]
pub struct RoomDb {
    pub pool: Arc<RwLock<HashMap<String, Room>>>,
}

impl RoomDb {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }
}

impl FromRef<AppState> for RoomDb {
    fn from_ref(input: &AppState) -> Self {
        input.room_db.clone()
    }
}

#[derive(Debug, Clone)]
pub struct UserDb {
    pub pool: PgPool,
}

impl UserDb {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: PgPool::connect(database_url).await?,
        })
    } 
}

impl FromRef<AppState> for UserDb {
    fn from_ref(input: &AppState) -> Self {
        input.user_db.clone()
    }
}
