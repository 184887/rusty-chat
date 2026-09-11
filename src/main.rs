use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::{Router, routing::get};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;
mod protocol;
use protocol::{ClientMessage, ServerMessage};

struct AppState {
    tx: broadcast::Sender<ServerMessage>,
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    // Handle WebSocket connections here
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();
    let mut username: Option<String> = None;

    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(ClientMessage::Join { username: name }) => {
                                username = Some(name.clone());
                                let _ = state.tx.send(ServerMessage::UserJoined { username: name });
                            }
                            Ok(ClientMessage::Chat { text }) => {
                                if let Some(name) = &username {
                                    let _ = state.tx.send(ServerMessage::Chat {
                                        username: name.clone(),
                                        text,
                                    });
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    _ => break,
                }
            }
            msg = rx.recv() => {
                match msg {
                    Ok(msg) => {
                        let json = serde_json::to_string(&msg).unwrap();
                        if sender.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    let (tx, _rx) = broadcast::channel(100);
    let state = Arc::new(AppState { tx });

    let m = ServerMessage::UserJoined {
        username: "magnus".into(),
    };
    println!("{}", serde_json::to_string(&m).unwrap());
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(state);

    println!("lytter på http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
