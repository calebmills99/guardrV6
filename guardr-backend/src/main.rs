mod config;
mod db;
mod models;
mod routes;
mod services;

use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = Router::new().nest("/api", routes::api_routes());

    let addr = "127.0.0.1:3000";
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
