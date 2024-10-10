use chat_api::{database::{rooms_db::RoomDb, users_db::UserDb}, route::app, AppError, AppState};
use tracing::info;

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
