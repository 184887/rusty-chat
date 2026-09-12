use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::protocol::{ClientMessage, ServerMessage};
use crate::state::AppState;

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
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
                Err(broadcast::error::RecvError::Lagged(_) ) => continue,
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
        }
    }

    if let Some(name) = username {
        let _ = state.tx.send(ServerMessage::UserLeft { username: name });
    }
}
