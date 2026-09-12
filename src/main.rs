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

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    let state = Arc::new(AppState {
        rooms: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/ws", get(ws::ws_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(state);

    println!("lytter på http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
