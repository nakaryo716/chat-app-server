use axum::response::IntoResponse;
use database::{rooms_db::RoomDb, users_db::UserDb};
use route::app;
use services::{auth::AuthError, user::UserServiciesError};
use tracing::info;

mod database;
mod handlers;
mod models;
mod route;
mod util;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("server started");
    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let room_db = RoomDb::new();
    let user_db = UserDb::connect(&database_url).await.unwrap();

    let app_state = AppState::new(room_db, user_db);
    let app = app::<AppError>(app_state);

    axum::serve(listener, app).await.unwrap();
}

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
