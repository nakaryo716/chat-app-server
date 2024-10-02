use database::users_db::UserDb;
use route::app;
use tracing::info;

mod database;
mod handlers;
mod middleware;
mod models;
mod route;
mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("server started");
    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let db = UserDb::connect(&database_url).await.unwrap();
    let app = app(db);

    axum::serve(listener, app).await.unwrap();
}
