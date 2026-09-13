mod protocol;
mod state;
mod ws;

use axum::{Router, routing::get};
use state::AppState;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    let db = SqlitePool::connect("sqlite:chat.db?mode=rwc").await.unwrap();
    sqlx::query(
    "CREATE TABLE IF NOT EXISTS messages (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        room TEXT NOT NULL,
        username TEXT NOT NULL,
        text TEXT NOT NULL,
        created_at INTEGER NOT NULL
    )"
)
.execute(&db)
.await
.unwrap();
    let state = Arc::new(AppState {
        rooms: Mutex::new(HashMap::new()),
        db,
    });

    let app = Router::new()
        .route("/ws", get(ws::ws_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(state);

    println!("lytter på http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
