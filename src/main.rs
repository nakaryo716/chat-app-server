use database::{rooms_db::RoomDb, users_db::UserDb};
use route::app;
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
    let app = app(app_state);

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
