use std::net::SocketAddr;

use axum_server::tls_openssl::OpenSSLConfig;
use tracing::info;

use chat_api::{
    database::{rooms_db::RoomDb, users_db::UserDb},
    route::app,
    AppError, AppState,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let config = OpenSSLConfig::from_pem_file("./signed-certs/cert.pem", "./signed-certs/key.pem")
        .expect("must set");

    info!("[system]Setting tls config....OK");
    println!("ok");

    let allow_origin = dotenvy::var("ALLOW_ORIGIN").unwrap();
    let origins = vec![allow_origin];
    info!("[system]Setting CORS policy....OK");

    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    let user_db = UserDb::connect(&database_url).await.unwrap();
    let room_db = RoomDb::new();
    let app_state = AppState::new(room_db, user_db);
    info!("[system]Setting Database....OK");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app = app::<AppError>(app_state, origins);
    info!("server running");

    axum_server::bind_openssl(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
