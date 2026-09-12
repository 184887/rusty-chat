mod protocol;
mod state;
mod ws;

use axum::{Router, routing::get};
use state::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;
use std::collections::HashMap;
use std::sync::Mutex;


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
