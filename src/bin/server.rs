use zircon::{room::database::RoomDb, route::app, users::database::UserDb, AppError, AppState};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("server started");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let allow_origin = dotenvy::var("ALLOW_ORIGIN").unwrap();
    let origins = vec![allow_origin];

    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    let user_db = UserDb::connect(&database_url).await.unwrap();
    let room_db = RoomDb::new();

    let app_state = AppState::new(room_db, user_db);
    let app = app::<AppError>(app_state, origins);

    axum::serve(listener, app).await.unwrap();
}
