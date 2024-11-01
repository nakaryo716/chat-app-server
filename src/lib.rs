use auth::services::AuthError;
use axum::response::IntoResponse;
use room::database::RoomDb;
use users::{database::UserDb, service::UserServiciesError};

mod auth;
mod chat;
pub mod handlers;
pub mod room;
pub mod route;
pub mod users;
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
