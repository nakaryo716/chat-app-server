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
    let listener = tokio::net::TcpListener::bind("0.0.0.1:8080").await.unwrap();
    
}
