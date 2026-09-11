use tokio::net::TcpListener;
use axum::{Router, routing::get}; 
use tower_http::services::ServeDir;
use axum::extract::ws::{Message,WebSocket, WebSocketUpgrade};
use axum::response::Response;
use std::sync::Arc;
use tokio::sync::broadcast;
use axum::extract::State;
use futures::{SinkExt, StreamExt};


struct AppState {
    tx: broadcast::Sender<String>,
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn ws_handler(ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>) ->  Response {
    // Handle WebSocket connections here
    ws.on_upgrade(move |socket| handle_socket(socket, state))
    
} 

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();
     loop {
    tokio::select! {
        msg = receiver.next() => {
            // fra klienten
             match msg {
            Some(Ok(Message::Text(text))) => {
            let _ = state.tx.send(text.to_string());
        }
        _ => break,
    }
        }
        msg = rx.recv() => {
            // fra kanalen
              match msg {
        Ok(text) => {
            if sender.send(Message::Text(text.into())).await.is_err() {
                break;
            }
        }
        Err(_) => break,
    }
        }
    }
     }}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    let (tx, _rx) = broadcast::channel(100);
    let state = Arc::new(AppState { tx });

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(state);
    

    println!("lytter på http://{addr}");
    axum::serve(listener, app).await.unwrap(); 

}

