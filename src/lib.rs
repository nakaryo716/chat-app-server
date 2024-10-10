use axum::response::IntoResponse;
use database::{rooms_db::RoomDb, users_db::UserDb};
use services::{auth::AuthError, user::UserServiciesError};

pub mod database;
pub mod handlers;
pub mod models;
pub mod route;
pub mod services;
pub mod util;

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
#[derive(Debug)]
pub enum AppError {
    Auth(AuthError),
    UserService(UserServiciesError),
}

impl From<AuthError> for AppError {
    fn from(error: AuthError) -> Self {
        AppError::Auth(error)
    }
}

impl From<UserServiciesError> for AppError {
    fn from(error: UserServiciesError) -> Self {
        AppError::UserService(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Auth(err) => err.into_response(),
            AppError::UserService(err) => err.into_response(),
        }
    }
}
